// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

#![warn(missing_docs)]

use crate::Error;
use crate::{Account, Address, Display, Network, Varint};
use bls::Serialize;
use rand::{rngs::OsRng, RngCore};
use secp256k1;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

pub type Result<T> = std::result::Result<T, Error>;

/// An identifier for a type of cryptographic key.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyTypeId(pub [u8; 4]);
impl Default for KeyTypeId {
    fn default() -> Self {
        KeyTypeId(*b"bls0")
    }
}
impl From<u32> for KeyTypeId {
    fn from(x: u32) -> Self {
        Self(x.to_le_bytes())
    }
}

impl From<KeyTypeId> for u32 {
    fn from(x: KeyTypeId) -> Self {
        u32::from_le_bytes(x.0)
    }
}

impl<'a> TryFrom<&'a str> for KeyTypeId {
    type Error = Error;
    fn try_from(x: &'a str) -> Result<Self> {
        let b = x.as_bytes();
        if b.len() != 4 {
            return Err(Error::InvalidLength);
        }
        let mut res = KeyTypeId::default();
        res.0.copy_from_slice(&b[0..4]);
        Ok(res)
    }
}

/// Known key types; this also functions as a global registry of key types for projects wishing to
/// avoid collisions with each other.
///
/// It's not universal in the sense that *all* key types need to be mentioned here, it's just a
/// handy place to put common key types.
pub mod key_types {
    use super::KeyTypeId;
    /// Key type for Babe module, build-in.
    pub const BLS: KeyTypeId = KeyTypeId(*b"bls0");
    /// Key type for Grandpa module, build-in.
    pub const SECP256K1: KeyTypeId = KeyTypeId(*b"secp");
    pub const ACTOR: KeyTypeId = KeyTypeId(*b"acto");
    pub const ID: KeyTypeId = KeyTypeId(*b"id00");
}

#[derive(Clone, Default)]
pub struct KeyPair {
    pub pubkey: Vec<u8>,
    pub privkey: Vec<u8>,
    pub key_type: KeyTypeId,
}

impl KeyPair {
    pub fn to_string(&self, key_type: KeyTypeId, net: Network) -> Result<String> {
        let addr: Address = match key_type {
            key_types::BLS => Account::BLS(self.pubkey.clone()).try_into()?,
            key_types::SECP256K1 => Account::SECP256K1(self.pubkey.clone()).try_into()?,
            key_types::ACTOR => Account::Actor(self.pubkey.clone()).try_into()?,
            _ => unreachable!("key types [bls, secp256k1, actor]"),
        };
        Ok(format!(
            "pubkey:{}\nprivkey:{}\naddress:{}",
            hex::encode(&self.pubkey),
            hex::encode(&self.privkey),
            addr.display(net)?
        ))
    }

    pub fn generate_key_pair(key_type: KeyTypeId) -> Result<Self> {
        let mut key = [0u8; 16];
        let pubkey: Vec<u8>;
        let privkey: Vec<u8>;

        OsRng.fill_bytes(&mut key);
        match key_type {
            key_types::BLS => {
                let private_key = bls::PrivateKey::generate(&mut OsRng);
                let public_key = private_key.public_key();
                pubkey = public_key.as_bytes();
                privkey = private_key.as_bytes();
            }
            key_types::SECP256K1 | key_types::ACTOR => {
                let secert = secp256k1::SecretKey::random(&mut OsRng);
                let public_key = secp256k1::PublicKey::from_secret_key(&secert);
                pubkey = public_key.serialize().to_vec();
                privkey = secert.serialize().to_vec();
            }
            key_types::ID => {
                let random_u64 = OsRng.next_u64();
                pubkey = Varint::U64(random_u64).into();
                privkey = Vec::default();
            }
            _ => return Err(Error::InvalidKeyType),
        }
        Ok(KeyPair {
            pubkey: pubkey,
            privkey: privkey,
            key_type: key_type,
        })
    }

    pub fn get_keypair_by_private(key_type: KeyTypeId, privkey: &[u8]) -> Result<Self> {
        let pubkey: Vec<u8>;
        match key_type {
            key_types::BLS => {
                let private_key = match bls::PrivateKey::from_bytes(privkey) {
                    Ok(p) => p,
                    Err(_) => return Err(Error::BytesConvertFailed),
                };

                let public_key = private_key.public_key();
                pubkey = public_key.as_bytes();
            }
            key_types::SECP256K1 => {
                let secert = secp256k1::SecretKey::parse_slice(privkey)?;
                let public_key = secp256k1::PublicKey::from_secret_key(&secert);
                pubkey = public_key.serialize().to_vec();
            }
            _ => return Err(Error::InvalidKeyType),
        }
        Ok(KeyPair {
            pubkey: pubkey,
            privkey: privkey.to_vec(),
            key_type: key_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Account, Address, Display, Network};
    use std::convert::TryInto;
    use std::str::FromStr;

    #[test]
    fn test_generate_key() {
        // Generate a key of a different type
        let keypair = KeyPair::generate_key_pair(KeyTypeId::default()).unwrap();
        let bls_addr: Address = Account::BLS(keypair.pubkey).try_into().unwrap();
        println!("{}\n", bls_addr.display(Network::Testnet).unwrap());

        let keypair = KeyPair::generate_key_pair(key_types::SECP256K1).unwrap();
        let secp_addr: Address = Account::SECP256K1(keypair.pubkey).try_into().unwrap();
        println!("{}\n", secp_addr.display(Network::Testnet).unwrap());
    }
}

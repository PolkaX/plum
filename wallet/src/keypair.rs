// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryInto;

use bls::Serialize;
use rand_core::{OsRng, RngCore};

use crate::account::Account;
use crate::address::{Address, Network};
use crate::error::{AddressError, Result};

/// The type of cryptographic key.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeyType {
    ///
    BLS,
    ///
    SECP256K1,
    ///
    Actor,
    ///
    ID,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::BLS
    }
}

///
#[derive(Clone, Default)]
pub struct KeyPair {
    ///
    pub pubkey: Vec<u8>,
    ///
    pub privkey: Vec<u8>,
    ///
    pub key_type: KeyType,
}

impl KeyPair {
    ///
    pub fn display(&self, key_type: KeyType, net: Network) -> Result<String> {
        let addr: Address = match key_type {
            KeyType::BLS => Account::BLS(self.pubkey.clone()).try_into()?,
            KeyType::SECP256K1 => Account::SECP256K1(self.pubkey.clone()).try_into()?,
            KeyType::Actor => Account::Actor(self.pubkey.clone()).try_into()?,
            _ => unreachable!("key types [bls, secp256k1, actor]"),
        };
        Ok(format!(
            "pubkey:{}\nprivkey:{}\naddress:{}",
            hex::encode(&self.pubkey),
            hex::encode(&self.privkey),
            addr.encode(net)?
        ))
    }

    ///
    pub fn gen_keypair(key_type: KeyType) -> Result<Self> {
        let (privkey, pubkey) = match key_type {
            KeyType::BLS => {
                let privkey = bls::PrivateKey::generate(&mut OsRng);
                let pubkey = privkey.public_key();
                (privkey.as_bytes(), pubkey.as_bytes())
            }
            KeyType::SECP256K1 | KeyType::Actor => {
                let seckey = secp256k1::SecretKey::random(&mut OsRng);
                let pubkey = secp256k1::PublicKey::from_secret_key(&seckey);
                (seckey.serialize().to_vec(), pubkey.serialize().to_vec())
            }
            KeyType::ID => {
                let random_u64 = OsRng.next_u64();
                let mut buf = unsigned_varint::encode::u64_buffer();
                (
                    vec![],
                    unsigned_varint::encode::u64(random_u64, &mut buf).to_vec(),
                )
            }
        };
        Ok(KeyPair {
            pubkey,
            privkey,
            key_type,
        })
    }

    ///
    pub fn gen_keypair_with_privkey(key_type: KeyType, privkey: &[u8]) -> Result<Self> {
        let pubkey = match key_type {
            KeyType::BLS => {
                let private_key = match bls::PrivateKey::from_bytes(privkey) {
                    Ok(p) => p,
                    Err(_) => return Err(AddressError::BytesConvertFailed),
                };
                let public_key = private_key.public_key();
                public_key.as_bytes()
            }
            KeyType::SECP256K1 => {
                let secret_key = secp256k1::SecretKey::parse_slice(privkey)?;
                let public_key = secp256k1::PublicKey::from_secret_key(&secret_key);
                public_key.serialize().to_vec()
            }
            _ => return Err(AddressError::InvalidKeyType),
        };
        Ok(KeyPair {
            pubkey,
            privkey: privkey.to_vec(),
            key_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() -> Result<()> {
        // Generate a key of a different type
        let keypair = KeyPair::gen_keypair(KeyType::BLS)?;
        let addr = Account::BLS(keypair.pubkey).try_into()?;
        println!("{}", addr.encode(Network::Test)?);

        let keypair = KeyPair::gen_keypair(KeyType::SECP256K1)?;
        let addr = Account::SECP256K1(keypair.pubkey).try_into()?;
        println!("{}", addr.encode(Network::Test)?);
        Ok(())
    }
}

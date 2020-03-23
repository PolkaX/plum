// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

use plum_address::Address;
use plum_crypto::{Signature, SignatureType};

use crate::error::{Result, WalletError};
use crate::keystore::{KeyInfo, KeyStore, KeyType};

const KNAME_PREFIX: &str = "wallet-";
const KDEFAULT: &str = "default";

///
#[derive(Clone, Debug)]
pub struct Key {
    info: KeyInfo,
    pubkey: Vec<u8>,
    address: Address,
}

impl Key {
    /// Create a new `Key` with given `KeyInfo`.
    pub fn new(info: KeyInfo) -> Result<Self> {
        match info.ty.clone().try_into()? {
            SignatureType::Secp256k1 => {
                let seckey = secp256k1::SecretKey::parse_slice(&info.privkey)?;
                let pubkey = secp256k1::PublicKey::from_secret_key(&seckey)
                    .serialize()
                    .to_vec();
                let address = Address::new_secp256k1_addr(&pubkey)?;
                Ok(Key {
                    info,
                    pubkey,
                    address,
                })
            }
            SignatureType::Bls => {
                use bls::Serialize;
                let privkey = bls::PrivateKey::from_bytes(&info.privkey)?;
                let pubkey = privkey.public_key().as_bytes();
                let address = Address::new_bls_addr(&pubkey)?;
                Ok(Key {
                    info,
                    pubkey,
                    address,
                })
            }
        }
    }
}

///
pub struct Wallet<KS: KeyStore> {
    keys: HashMap<Address, Key>,
    keystore: KS,
}

impl<KS: KeyStore> Wallet<KS> {
    /// Create a new `Wallet` with the given `KeyStore`.
    pub fn new(keystore: KS) -> Self {
        Self {
            keys: HashMap::new(),
            keystore,
        }
    }

    /// Create a new `Wallet` with the given `keys` and `KeyStore`.
    pub fn new_with_keys(keys: Vec<Key>, keystore: KS) -> Self {
        Self {
            keys: keys
                .into_iter()
                .map(|key| (key.address.clone(), key))
                .collect(),
            keystore,
        }
    }

    ///
    pub fn sign(&self, addr: &Address, msg: &[u8]) -> Result<Signature> {
        let key = self.find_key(addr).ok_or(WalletError::KeyNotFound)?;
        match key.info.ty.try_into()? {
            SignatureType::Secp256k1 => Ok(Signature::sign_secp256k1(key.info.privkey, msg)?),
            SignatureType::Bls => Ok(Signature::sign_bls(key.info.privkey, msg)?),
        }
    }

    ///
    pub fn has_key(&self) -> Result<()> {
        Ok(())
    }

    fn find_key(&self, addr: &Address) -> Option<Key> {
        if let Some(key) = self.keys.get(addr) {
            return Some(key.clone());
        }
        None
    }

    /// Generate address by the key type.
    pub fn generate_key(&mut self, key_type: SignatureType) -> Result<Address> {
        let key = generate_key(key_type)?;

        if let Err(_) = self
            .keystore
            .put(format!("{}{}", KNAME_PREFIX, key.address), key.info.clone())
        {
            return Err(WalletError::KeyStore);
        }
        let address = key.address.clone();
        self.keys.insert(address.clone(), key);

        // TODO

        Ok(address)
    }

    /// List all addresses in keystore.
    pub fn list_addresses(&self) -> Result<Vec<Address>> {
        let mut addresses = match self.keystore.list() {
            Ok(addresses) => addresses,
            Err(_) => return Err(WalletError::KeyStore),
        };
        addresses.sort();
        let mut res = Vec::with_capacity(addresses.len());
        for address in &addresses {
            if address.starts_with(KNAME_PREFIX) {
                let addr = Address::from_str(address.trim_start_matches(KNAME_PREFIX))?;
                res.push(addr);
            }
        }
        Ok(res)
    }

    /// Export the key-info by the address.
    pub fn export(&self, addr: &Address) -> Result<KeyInfo> {
        let key = self.find_key(addr).ok_or(WalletError::KeyNotFound)?;
        // TODO: export from keystore
        Ok(key.info)
    }

    /// Import address by key info.
    pub fn import(&mut self, info: KeyInfo) -> Result<Address> {
        let key = Key::new(info)?;
        match self
            .keystore
            .put(format!("{}{}", KNAME_PREFIX, key.address), key.info.clone())
        {
            Ok(()) => Ok(key.address),
            Err(_) => Err(WalletError::KeyNotFound),
        }
    }

    /// Get the address of default key info in keystore.
    pub fn get_default(&self) -> Result<Address> {
        todo!()
    }

    /// Set the default
    pub fn set_default(&mut self, _address: Address) -> Result<()> {
        // TODO
        Ok(())
    }
}

pub fn generate_key(key_type: SignatureType) -> Result<Key> {
    match key_type {
        SignatureType::Secp256k1 => {
            let secret = secp256k1::SecretKey::random(&mut rand::rngs::OsRng);
            let privkey = secret.serialize().to_vec();
            Key::new(KeyInfo {
                ty: KeyType::Secp256k1,
                privkey,
            })
        }
        SignatureType::Bls => {
            use bls::Serialize;
            let privkey = bls::PrivateKey::generate(&mut rand::rngs::OsRng);
            let privkey = privkey.as_bytes();
            Key::new(KeyInfo {
                ty: KeyType::Bls,
                privkey,
            })
        }
    }
}

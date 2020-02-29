// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;
use std::str::FromStr;

use plum_address::{Address, NETWORK_DEFAULT};

use crate::error::{Result, WalletError};
use crate::keystore::{KeyInfo, KeyStore, KeyType, Signature};

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
        match info.ty {
            KeyType::SECP256K1 => {
                let seckey = secp256k1::SecretKey::parse_slice(&info.privkey)?;
                let pubkey = secp256k1::PublicKey::from_secret_key(&seckey)
                    .serialize()
                    .to_vec();
                let address = Address::new_secp256k1_addr(NETWORK_DEFAULT, &pubkey)?;
                Ok(Key {
                    info,
                    pubkey,
                    address,
                })
            }
            KeyType::BLS => {
                use bls::Serialize;
                let privkey = match bls::PrivateKey::from_bytes(&info.privkey) {
                    Ok(privkey) => privkey,
                    Err(_) => return Err(WalletError::BLS),
                };
                let pubkey = privkey.public_key().as_bytes();
                let address = Address::new_bls_addr(NETWORK_DEFAULT, &pubkey)?;
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
        Wallet {
            keys: HashMap::new(),
            keystore,
        }
    }

    /// Create a new `Wallet` with the given `keys` and `KeyStore`.
    pub fn new_with_keys(keys: Vec<Key>, keystore: KS) -> Self {
        Wallet {
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
        match key.info.ty {
            KeyType::SECP256K1 => {
                let signature = sigs::secp256k1_sign(&key.info.privkey, msg)?;
                Ok(Signature {
                    ty: KeyType::SECP256K1,
                    data: signature,
                })
            }
            KeyType::BLS => {
                let signature = sigs::bls_sign(key.info.privkey.as_slice(), msg)?;
                Ok(Signature {
                    ty: KeyType::BLS,
                    data: signature,
                })
            }
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

        //        self.keystore
        //            .get(format!("{}{}", KNAME_PREFIX, addr.encode(NETWORK)))
        None
    }

    /// Generate address by the key type.
    pub fn generate_key(&mut self, key_type: KeyType) -> Result<Address> {
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

pub fn generate_key(key_type: KeyType) -> Result<Key> {
    match key_type {
        KeyType::SECP256K1 => {
            let seckey = secp256k1::SecretKey::random(&mut rand::rngs::OsRng);
            Key::new(KeyInfo {
                ty: KeyType::SECP256K1,
                privkey: seckey.serialize().to_vec(),
            })
        }
        KeyType::BLS => {
            use bls::Serialize;
            let privkey = bls::PrivateKey::generate(&mut rand::rngs::OsRng);
            Key::new(KeyInfo {
                ty: KeyType::BLS,
                privkey: privkey.as_bytes(),
            })
        }
    }
}

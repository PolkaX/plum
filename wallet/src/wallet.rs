// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use parking_lot::RwLock;

use plum_address::Address;
use plum_crypto::Signature;

use crate::error::{Result, WalletError};
use crate::keystore::{KeyInfo, KeyStore, KeyType};

const WALLET_NAME_PREFIX: &str = "wallet-";
const DEFAULT_KEY_NAME: &str = "default";

///
#[derive(PartialEq, Clone, Debug)]
pub struct Key {
    /// key type and private key.
    pub info: KeyInfo,
    /// public key
    pub pubkey: Vec<u8>,
    /// address
    pub address: Address,
}

impl Key {
    /// Create a new `Key` with given `KeyInfo`.
    pub fn new(info: KeyInfo) -> Result<Self> {
        match info.ty {
            KeyType::Secp256k1 => Self::new_secp256k1(info.privkey),
            KeyType::Bls => Self::new_bls(info.privkey),
            _ => Err(WalletError::UnknownKeyType),
        }
    }

    /// Create a new secp256k1 `Key` with given private key.
    pub fn new_secp256k1<K: AsRef<[u8]>>(privkey: K) -> Result<Self> {
        let seckey = secp256k1::SecretKey::parse_slice(privkey.as_ref())?;
        let pubkey = secp256k1::PublicKey::from_secret_key(&seckey)
            .serialize()
            .to_vec();
        let address = Address::new_secp256k1_addr(&pubkey)?;
        Ok(Key {
            info: KeyInfo {
                ty: KeyType::Secp256k1,
                privkey: seckey.serialize().to_vec(),
            },
            pubkey,
            address,
        })
    }

    /// Create a new bls `Key` with given private key.
    pub fn new_bls<K: AsRef<[u8]>>(privkey: K) -> Result<Self> {
        use bls::Serialize;
        let seckey = bls::PrivateKey::from_bytes(privkey.as_ref())?;
        let pubkey = seckey.public_key().as_bytes();
        let address = Address::new_bls_addr(&pubkey)?;
        Ok(Key {
            info: KeyInfo {
                ty: KeyType::Bls,
                privkey: seckey.as_bytes(),
            },
            pubkey,
            address,
        })
    }
}

/// A Wrapper of WalletImpl.
pub struct Wallet<KS: KeyStore> {
    imp: Arc<RwLock<WalletImpl<KS>>>,
}

impl<KS: KeyStore> Wallet<KS> {
    /// Create a new `Wallet` with the given `KeyStore`.
    pub fn new(keystore: KS) -> Self {
        Self {
            imp: Arc::new(RwLock::new(WalletImpl::new(keystore))),
        }
    }

    /// Create a new `Wallet` with the given `keys` and `KeyStore`.
    pub fn new_with_keys(keys: Vec<Key>, keystore: KS) -> Self {
        Self {
            imp: Arc::new(RwLock::new(WalletImpl::new_with_keys(keys, keystore))),
        }
    }

    /// Sign the message with the private key found by the given address in the key store.
    pub fn sign<M: AsRef<[u8]>>(&self, addr: &Address, msg: M) -> Result<Signature> {
        let wallet = self.imp.read();
        wallet.sign(addr, msg)
    }

    /// Export the key info by the address.
    pub fn export(&self, addr: &Address) -> Option<KeyInfo> {
        let wallet = self.imp.read();
        wallet.export(addr)
    }

    /// Import address by key info.
    pub fn import(&mut self, key_info: KeyInfo) -> Result<Address> {
        let mut wallet = self.imp.write();
        wallet.import(key_info)
    }

    /// List all addresses in keystore.
    pub fn list_addrs(&self) -> Result<Vec<Address>> {
        let wallet = self.imp.read();
        wallet.list_addrs()
    }

    /// Get the address of default key info in the keystore.
    pub fn get_default(&self) -> Result<Option<Address>> {
        let wallet = self.imp.read();
        wallet.get_default()
    }

    /// Set the default key info of the keystore according to the address.
    pub fn set_default(&mut self, addr: &Address) -> Result<()> {
        let mut wallet = self.imp.write();
        wallet.set_default(addr)
    }

    /// Generate an address by the key type randomly.
    pub fn generate_key(&mut self, key_type: KeyType) -> Result<Address> {
        let mut wallet = self.imp.write();
        wallet.generate_key(key_type)
    }

    /// Whether the addr exists in the wallet.
    pub fn has_key(&self, addr: &Address) -> bool {
        let wallet = self.imp.read();
        wallet.has_key(addr)
    }
}

struct WalletImpl<KS: KeyStore> {
    // mem: address => Key
    keys: HashMap<Address, Key>,
    // keystore:
    // 1. string (another format of address) => KeyInfo
    // 2. "default" => KeyInfo
    keystore: KS,
}

impl<KS: KeyStore> WalletImpl<KS> {
    /// Create a new `Wallet` with the given `KeyStore`.
    fn new(keystore: KS) -> Self {
        Self {
            keys: HashMap::new(),
            keystore,
        }
    }

    /// Create a new `Wallet` with the given `keys` and `KeyStore`.
    fn new_with_keys(keys: Vec<Key>, keystore: KS) -> Self {
        Self {
            keys: keys
                .into_iter()
                .map(|key| (key.address.clone(), key))
                .collect(),
            keystore,
        }
    }

    /// Sign the message with the private key found by the given address in the key store.
    fn sign<M: AsRef<[u8]>>(&self, addr: &Address, msg: M) -> Result<Signature> {
        match self.find_key(addr) {
            Some(key) => match key.info.ty {
                KeyType::Secp256k1 => {
                    Ok(Signature::sign_secp256k1(&key.info.privkey, msg.as_ref())?)
                }
                KeyType::Bls => Ok(Signature::sign_bls(&key.info.privkey, msg.as_ref())?),
                _ => Err(WalletError::UnknownKeyType),
            },
            None => Err(WalletError::KeyStore(format!(
                "key `{}{}` not found",
                WALLET_NAME_PREFIX, addr
            ))),
        }
    }

    fn find_key(&self, addr: &Address) -> Option<&Key> {
        self.keys.get(addr)
    }

    /// Export the key info by the address.
    fn export(&self, addr: &Address) -> Option<KeyInfo> {
        match self.find_key(addr) {
            Some(key) => Some(key.info.clone()),
            None => None,
        }
    }

    /// Import address by key info.
    fn import(&mut self, key_info: KeyInfo) -> Result<Address> {
        let key = Key::new(key_info)?;
        // import the key (with the `wallet-xxx`format) and key info.
        match self.keystore.put(
            format!("{}{}", WALLET_NAME_PREFIX, key.address),
            key.info.clone(),
        ) {
            Ok(_) => {
                // update the key in the memory
                let address = key.address.clone();
                self.keys.insert(address.clone(), key);
                Ok(address)
            }
            Err(err) => Err(WalletError::KeyStore(err.to_string())),
        }
    }

    /// List all addresses in keystore.
    fn list_addrs(&self) -> Result<Vec<Address>> {
        let mut addrs = match self.keystore.list() {
            Ok(addrs) => addrs,
            Err(err) => return Err(WalletError::KeyStore(err.to_string())),
        };
        addrs.sort();

        let mut res = Vec::with_capacity(addrs.len());
        // will list all addresses with the format (`wallet-xxx`, `xxx` is the actual address string).
        // won't list the default address.
        for addr in &addrs {
            if addr.starts_with(WALLET_NAME_PREFIX) {
                let addr_str = addr.trim_start_matches(WALLET_NAME_PREFIX);
                let addr = Address::from_str(addr_str)?;
                res.push(addr);
            }
        }
        Ok(res)
    }

    /// Get the address of default key info in the keystore.
    fn get_default(&self) -> Result<Option<Address>> {
        match self
            .keystore
            .get(DEFAULT_KEY_NAME)
            .map_err(|err| WalletError::KeyStore(err.to_string()))?
        {
            Some(key_info) => match key_info.ty {
                KeyType::Secp256k1 => Ok(Some(Key::new_secp256k1(&key_info.privkey)?.address)),
                KeyType::Bls => Ok(Some(Key::new_bls(&key_info.privkey)?.address)),
                _ => Err(WalletError::UnknownKeyType),
            },
            None => Ok(None),
        }
    }

    /// Set the default key info of the keystore according to the address.
    fn set_default(&mut self, addr: &Address) -> Result<()> {
        // get key info from the memory according to the address.
        let key_info = self
            .keys
            .get(addr)
            .expect("no `wallet-` prefix key in the keystore")
            .info
            .clone();

        // use the key info from the memory as the default key info.
        self.keystore
            .delete(DEFAULT_KEY_NAME)
            .map_err(|err| WalletError::KeyStore(err.to_string()))?;
        let _default = self
            .keystore
            .put(DEFAULT_KEY_NAME.to_string(), key_info)
            .map_err(|err| WalletError::KeyStore(err.to_string()))?;
        Ok(())
    }

    /// Generate an address by the key type randomly.
    fn generate_key(&mut self, key_type: KeyType) -> Result<Address> {
        let key = generate_key(key_type)?;

        // generate a random key info and save it into key store and memory.
        let old = self
            .keystore
            .put(
                format!("{}{}", WALLET_NAME_PREFIX, key.address),
                key.info.clone(),
            )
            .map_err(|err| WalletError::KeyStore(err.to_string()))?;
        assert_eq!(old, None);
        let address = key.address.clone();
        let key_info = key.info.clone();
        let old = self.keys.insert(address.clone(), key);
        assert_eq!(old, None);

        // use the random key info as the default key info randomly if no default key info exists in the key store.
        if self
            .keystore
            .get(DEFAULT_KEY_NAME)
            .map_err(|err| WalletError::KeyStore(err.to_string()))?
            .is_none()
        {
            let old_default = self
                .keystore
                .put(DEFAULT_KEY_NAME.to_string(), key_info)
                .map_err(|err| WalletError::KeyStore(err.to_string()))?;
            assert_eq!(old_default, None);
        }

        Ok(address)
    }

    /// Whether the addr exists in the wallet.
    fn has_key(&self, addr: &Address) -> bool {
        self.find_key(addr).is_some()
    }
}

/// Generate a key with the given key type randomly.
pub fn generate_key(key_type: KeyType) -> Result<Key> {
    match key_type {
        KeyType::Secp256k1 => {
            let secret = secp256k1::SecretKey::random(&mut rand::rngs::OsRng);
            let privkey = secret.serialize().to_vec();
            Key::new_secp256k1(privkey)
        }
        KeyType::Bls => {
            use bls::Serialize;
            let privkey = bls::PrivateKey::generate(&mut rand::rngs::OsRng);
            let privkey = privkey.as_bytes();
            Key::new_bls(privkey)
        }
        _ => Err(WalletError::UnknownKeyType),
    }
}

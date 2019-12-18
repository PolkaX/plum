// Copyright 2019 PolkaX

//! Keystore (and session key management) for ed25519 based chains like Polkadot.

#![warn(missing_docs)]

use std::{collections::HashMap, path::PathBuf, fs::{self, File}, io::{self, Write}, sync::Arc};
use parking_lot::RwLock;
use rand::{RngCore, rngs::OsRng};
use bls::Serialize;
use secp256k1;
use crate::crypto::{key_types, KeyTypeId};
use crate::address::{Address, Account, Network, Display};
use std::convert::TryInto;
use std::str::FromStr;

/// Keystore pointer
pub type KeyStorePtr = Arc<RwLock<Store>>;

/// Keystore error.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// JSON error.
    Json(serde_json::Error),
    /// Invalid key type
    #[display(fmt="Invalid key type")]
    InvalidKeyType,
    /// Keystore unavailable
    #[display(fmt="Keystore unavailable")]
    Unavailable,
}

/// Keystore Result
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct KeyPair {
    pubkey: Vec<u8>,
    privkey: Vec<u8>,
    key_type: KeyTypeId,
}

impl KeyPair {
    fn list(self, key_type: KeyTypeId, net: Network) -> Result<String> {
        let addr: Address;
        match key_type.clone() {
            key_types::BLS => {
                addr = Account::BLS(self.pubkey.clone()).try_into().unwrap()
            },
            key_types::SECP256K1 => {
                addr = Account::SECP256K1(self.pubkey.clone())
                    .try_into()
                    .unwrap()
            },
            _ => return Err(Error::InvalidKeyType),
        }
        let addr = addr.display(net);
        let addr = format!("key_type:{:?}\nPublicKey:{:?}\nPrivateKey:{:?}\naddress:{:?}\n",key_type, self.pubkey, self.privkey, addr);
        Ok(addr)
    }

    fn to_string(self) -> String {
        format!("PublicKey:{:?}\nPrivateKey:{:?}\n", self.pubkey, self.privkey)
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

            },
            key_types::SECP256K1 => {
                let secert = secp256k1::SecretKey::random(& mut OsRng);
                let public_key = secp256k1::PublicKey::from_secret_key(&secert);
                pubkey = public_key.serialize().to_vec();
                privkey = secert.serialize().to_vec();
            },
            _ => return Err(Error::InvalidKeyType)
        }
        Ok(KeyPair {
            pubkey: pubkey,
            privkey: privkey,
            key_type: key_type,
        })
    }
}

/// Key store.
///
/// Stores key pairs in a file system store + short lived key pairs in memory.
///
/// Every pair that is being generated by a `seed`, will be placed in memory.
pub struct Store {
    path: PathBuf,
    additional: HashMap<(KeyTypeId, Vec<u8>), Vec<u8>>,
    //password: Option<Protected<String>>,
}

impl Store {

    /// Open the store at the given path.
    ///
    /// Optionally takes a password that will be used to encrypt/decrypt the keys.
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<KeyStorePtr> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let instance = Self { path, additional: HashMap::new() };
        Ok(Arc::new(RwLock::new(instance)))
    }

    /// Generate a new key.
    ///
    /// Places it into the file system store.
    pub fn generate_key(&self, key_type: KeyTypeId) -> Result<KeyPair> {
        let pair = KeyPair::generate_key_pair(key_type)?;
        let mut file = File::create(self.key_file_path(pair.pubkey.as_slice(), key_type))?;
        serde_json::to_writer(&file, &pair.clone().to_string())?;
        file.flush()?;
        Ok(pair)
    }

    /// Insert a new key with anonymous crypto.
    ///
    /// Places it into the file system store.
    fn insert_unknown(&self, key_type: KeyTypeId, suri: &str, public: &[u8]) -> Result<()> {
        let mut file = File::create(self.key_file_path(public, key_type)).map_err(Error::Io)?;
        serde_json::to_writer(&file, &suri).map_err(Error::Json)?;
        file.flush().map_err(Error::Io)?;
        Ok(())
    }

    /// Returns the file path for the given public key and key type.
    fn key_file_path(&self, public: &[u8], key_type: KeyTypeId) -> PathBuf {
        let mut buf = self.path.clone();
        let key_type = hex::encode(key_type.0);
        let key = hex::encode(public);
        buf.push(key_type + key.as_str());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::str::FromStr;
    use crate::address::{Network, Account, Display};
    use std::convert::TryInto;

    #[test]
    fn test_generate_key() {
        let mut map: HashMap<(KeyTypeId, Vec<u8>), Vec<u8>> = HashMap::new();
        let st = Store {
            path: PathBuf::from_str("./tmp").unwrap(),
            additional: map,
        };
        let keypair = st.generate_bls_key(KeyTypeId::default()).unwrap();
        println!("{}",keypair.clone().to_string());
        let addr: crate::address::Address =
            crate::address::Account::BLS(keypair.bls_pubkey.as_bytes()).try_into().unwrap();
        println!("{}", addr.display(Network::Testnet));
    }
//
//    #[test]
//    fn basic_store() {
//        let temp_dir = TempDir::new().unwrap();
//        let store = Store::open(temp_dir.path(), None).unwrap();
//
//        assert!(store.read().public_keys::<ed25519::AppPublic>().unwrap().is_empty());
//
//        let key: ed25519::AppPair = store.write().generate().unwrap();
//        let key2: ed25519::AppPair = store.read().key_pair(&key.public()).unwrap();
//
//        assert_eq!(key.public(), key2.public());
//
//        assert_eq!(store.read().public_keys::<ed25519::AppPublic>().unwrap()[0], key.public());
//    }
//
//    #[test]
//    fn test_insert_ephemeral_from_seed() {
//        let temp_dir = TempDir::new().unwrap();
//        let store = Store::open(temp_dir.path(), None).unwrap();
//
//        let pair: ed25519::AppPair = store
//            .write()
//            .insert_ephemeral_from_seed("0x3d97c819d68f9bafa7d6e79cb991eebcd77d966c5334c0b94d9e1fa7ad0869dc")
//            .unwrap();
//        assert_eq!(
//            "5DKUrgFqCPV8iAXx9sjy1nyBygQCeiUYRFWurZGhnrn3HJCA",
//            pair.public().to_ss58check()
//        );
//
//        drop(store);
//        let store = Store::open(temp_dir.path(), None).unwrap();
//        // Keys generated from seed should not be persisted!
//        assert!(store.read().key_pair::<ed25519::AppPair>(&pair.public()).is_err());
//    }
//
//    #[test]
//    fn public_keys_are_returned() {
//        let temp_dir = TempDir::new().unwrap();
//        let store = Store::open(temp_dir.path(), None).unwrap();
//
//        let mut public_keys = Vec::new();
//        for i in 0..10 {
//            public_keys.push(store.write().generate::<ed25519::AppPair>().unwrap().public());
//            public_keys.push(store.write().insert_ephemeral_from_seed::<ed25519::AppPair>(
//                &format!("0x3d97c819d68f9bafa7d6e79cb991eebcd7{}d966c5334c0b94d9e1fa7ad0869dc", i),
//            ).unwrap().public());
//        }
//
//        // Generate a key of a different type
//        store.write().generate::<sr25519::AppPair>().unwrap();
//
//        public_keys.sort();
//        let mut store_pubs = store.read().public_keys::<ed25519::AppPublic>().unwrap();
//        store_pubs.sort();
//
//        assert_eq!(public_keys, store_pubs);
//    }
//
//    #[test]
//    fn store_unknown_and_extract_it() {
//        let temp_dir = TempDir::new().unwrap();
//        let store = Store::open(temp_dir.path(), None).unwrap();
//
//        let secret_uri = "//Alice";
//        let key_pair = sr25519::AppPair::from_string(secret_uri, None).expect("Generates key pair");
//
//        store.write().insert_unknown(
//            SR25519,
//            secret_uri,
//            key_pair.public().as_ref(),
//        ).expect("Inserts unknown key");
//
//        let store_key_pair = store.read().key_pair_by_type::<sr25519::AppPair>(
//            &key_pair.public(),
//            SR25519,
//        ).expect("Gets key pair from keystore");
//
//        assert_eq!(key_pair.public(), store_key_pair.public());
//    }
//
//    #[test]
//    fn store_ignores_files_with_invalid_name() {
//        let temp_dir = TempDir::new().unwrap();
//        let store = Store::open(temp_dir.path(), None).unwrap();
//
//        let file_name = temp_dir.path().join(hex::encode(&SR25519.0[..2]));
//        fs::write(file_name, "test").expect("Invalid file is written");
//
//        assert!(
//            store.read().public_keys_by_type::<sr25519::AppPublic>(SR25519).unwrap().is_empty(),
//        );
//    }
}

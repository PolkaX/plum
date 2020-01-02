// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

const KEYSTORE_PATH: &str = "/.plum/keystore/";

///
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeyType {
    ///
    SECP256K1,
    ///
    BLS,
}

/// KeyInfo is used for storing keys in KeyStore.
#[derive(Clone, Debug)]
pub struct KeyInfo {
    /// The key type.
    pub ty: KeyType,
    /// The private key corresponding to key type.
    pub privkey: Vec<u8>,
}

/// KeyStore is used for storing secret keys.
pub trait KeyStore {
    /// The error type that keystore operations may return.
    type Error;

    /// List lists all the keys stored in the KeyStore.
    fn list(&self) -> Result<Vec<String>, Self::Error>;
    /// Get gets a key out of keystore and returns KeyInfo corresponding to named key.
    fn get(&self, _: &str) -> Result<KeyInfo, Self::Error>;
    /// Put saves a key info under given name.
    fn put(&mut self, _: String, _: KeyInfo) -> Result<(), Self::Error>;
    /// Delete removes a key from keystore.
    fn delete(&mut self, _: String) -> Result<(), Self::Error>;
}

///
#[derive(PartialEq, Eq, Clone)]
pub struct Signature {
    ///
    pub ty: KeyType,
    ///
    pub data: Vec<u8>,
}

/*
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// Keystore pointer
pub type KeyStorePtr = Arc<RwLock<Store>>;

/// Key store.
#[derive(Default)]
pub struct Store {
    pub path: PathBuf,
    //password: Option<Protected<String>>,
}

impl Store {
    /// Open the store at the given path.
    pub fn open(&self) -> Result<KeyStorePtr> {
        fs::create_dir_all(&self.path)?;
        let path = self.path.clone();
        let instance = Self { path };
        Ok(Arc::new(RwLock::new(instance)))
    }

    /// Generate a new key.
    pub fn generate_key(&self, key_type: KeyType) -> Result<KeyPair> {
        let pair = KeyPair::gen_keypair(key_type)?;
        let mut file = File::create(self.key_file_path(pair.pubkey.as_slice(), key_type))?;
        println!("{:?}", file);
        serde_json::to_writer(&file, &hex::encode(&pair.clone().privkey))?;
        file.flush()?;
        Ok(pair)
    }

    ///
    pub fn import_key(&self, key_type: KeyType, privkey: &[u8]) -> Result<KeyPair> {
        let pair = KeyPair::gen_keypair_with_privkey(key_type, privkey)?;
        let mut file = File::create(self.key_file_path(pair.pubkey.as_slice(), key_type))?;
        serde_json::to_writer(&file, &hex::encode(&pair.clone().privkey))?;
        file.flush()?;
        Ok(pair)
    }

    /// Returns the file path for the given public key and key type.
    fn key_file_path(&self, public: &[u8], key_type: KeyType) -> PathBuf {
        let mut buf = self.path.clone();
        let key_type = hex::encode(key_type.0);
        let key = hex::encode(public);
        buf.push(key_type + key.as_str());
        buf
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::str::FromStr;

    use address::{Account, Address, Network};
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_generate_key() {
        let st = Store {
            path: PathBuf::from_str("./tmp/").unwrap(),
        };
        let _ = st.open().unwrap();

        // Generate a key of a different type
        let keypair = st.generate_key(KeyType::default()).unwrap();
        let bls_addr: Address = Account::BLS(keypair.pubkey).try_into().unwrap();
        println!("{}\n", bls_addr.encode(Network::Test).unwrap());

        let keypair = st.generate_key(key_types::SECP256K1).unwrap();
        let secp_addr: Address = Account::SECP256K1(keypair.pubkey).try_into().unwrap();
        println!("{}\n", secp_addr.encode(Network::Test).unwrap());
    }
}
*/

// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

//! Keystore (and session key management) for ed25519 based chains like Polkadot.

#![warn(missing_docs)]

use crate::error::Error;
use address::keypair::{key_types, KeyPair, KeyTypeId};
use parking_lot::RwLock;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

/// Keystore pointer
pub type KeyStorePtr = Arc<RwLock<Store>>;
pub type Result<T> = std::result::Result<T, Error>;

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
        let instance = Self { path: path };
        Ok(Arc::new(RwLock::new(instance)))
    }

    /// Generate a new key.
    pub fn generate_key(&self, key_type: KeyTypeId) -> Result<KeyPair> {
        let pair = KeyPair::generate_key_pair(key_type)?;
        let mut file = File::create(self.key_file_path(pair.pubkey.as_slice(), key_type))?;
        println!("{:?}", file);
        serde_json::to_writer(&file, &hex::encode(&pair.clone().privkey))?;
        file.flush()?;
        Ok(pair)
    }

    pub fn import_key(&self, key_type: KeyTypeId, privkey: &[u8]) -> Result<KeyPair> {
        let pair = KeyPair::get_keypair_by_private(key_type, privkey)?;
        let mut file = File::create(self.key_file_path(pair.pubkey.as_slice(), key_type))?;
        serde_json::to_writer(&file, &hex::encode(&pair.clone().privkey))?;
        file.flush()?;
        Ok(pair)
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
    use address::{Account, Address, Display, Network};
    use std::convert::TryInto;
    use std::str::FromStr;
    use tempfile::TempDir;

    #[test]
    fn test_generate_key() {
        let st = Store {
            path: PathBuf::from_str("./tmp/").unwrap(),
        };
        let _ = st.open().unwrap();

        // Generate a key of a different type
        let keypair = st.generate_key(KeyTypeId::default()).unwrap();
        let bls_addr: Address = Account::BLS(keypair.pubkey).try_into().unwrap();
        println!("{}\n", bls_addr.display(Network::Testnet).unwrap());

        let keypair = st.generate_key(key_types::SECP256K1).unwrap();
        let secp_addr: Address = Account::SECP256K1(keypair.pubkey).try_into().unwrap();
        println!("{}\n", secp_addr.display(Network::Testnet).unwrap());
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert;
use std::fmt;

use serde::{de, ser, Deserialize, Serialize};
use thiserror::Error;

use plum_crypto::SignatureType;

const KEYSTORE_PATH: &str = "/.plum/keystore/";

///
#[derive(Debug, Error)]
pub enum KeyTypeError {
    ///
    #[error("unknown signature type for key. type number: {0}")]
    UnknownSignatureType(u8),
    ///
    #[error("can't convert current `KeyType` into `SignatureType`. KeyType:{0:?}")]
    UnsupportedSignatureType(KeyType),
}

///
#[derive(Eq, PartialEq, Clone)]
pub enum KeyType {
    ///
    Secp256k1,
    ///
    Bls,
    ///
    Libp2p,
    ///
    Other(String),
}

impl fmt::Debug for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyType::Secp256k1 => "secp256k1",
            KeyType::Bls => "bls",
            KeyType::Libp2p => "libp2p-host",
            KeyType::Other(s) => s.as_str(),
        };
        f.write_str(s)
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<SignatureType> for KeyType {
    fn from(s: SignatureType) -> Self {
        match s {
            SignatureType::Secp256k1 => KeyType::Secp256k1,
            SignatureType::Bls => KeyType::Bls,
        }
    }
}

impl convert::TryInto<SignatureType> for KeyType {
    type Error = KeyTypeError;

    fn try_into(self) -> Result<SignatureType, Self::Error> {
        match self {
            KeyType::Secp256k1 => Ok(SignatureType::Secp256k1),
            KeyType::Bls => Ok(SignatureType::Bls),
            _ => Err(KeyTypeError::UnsupportedSignatureType(self)),
        }
    }
}

impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "secp256k1" => KeyType::Secp256k1,
            "bls" => KeyType::Bls,
            "libp2p-host" => KeyType::Libp2p,
            _ => KeyType::Other(s),
        })
    }
}

/// KeyInfo is used for storing keys in KeyStore.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyInfo {
    /// The key type.
    #[serde(rename = "Type")]
    pub ty: KeyType,
    /// The private key corresponding to key type.
    #[serde(
        rename = "PrivateKey",
        serialize_with = "serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub privkey: Vec<u8>,
}

pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + AsRef<[u8]>,
    S: ser::Serializer,
{
    String::from_utf8_lossy(bytes.as_ref()).serialize(serializer)
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

use std::{collections::HashMap, path::PathBuf, fs::{self, File}, io::{self, Write}, sync::Arc};
use crypto::KeyTypeId;
use parking_lot::RwLock;

mod address;
mod crypto;
mod keystore;
//
// Generate
// Sign
// Remove
//walletNew,
//walletList,
//walletBalance,

/// wallet pointer
pub type WalletPtr = Arc<RwLock<Wallet>>;

/// Keystore error.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// JSON error.
    Json(serde_json::Error),
    /// Invalid password.
    #[display(fmt="Invalid password")]
    InvalidPassword,
    /// Invalid BIP39 phrase
    #[display(fmt="Invalid recovery phrase (BIP39) data")]
    InvalidPhrase,
    /// Invalid seed
    #[display(fmt="Invalid seed")]
    InvalidSeed,
    /// Keystore unavailable
    #[display(fmt="Keystore unavailable")]
    Unavailable,
}
/// wallet Result
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

pub struct Wallet {
    keys: HashMap<(KeyTypeId, Vec<u8>), Vec<u8>>,
    path: PathBuf
    //password: Option<Protected<String>>,
}

impl Wallet {
    pub fn new_address(key_type: KeyTypeId) -> Result<String> {
        let pair = keystore::KeyPair::generate_key_pair(key_type)?;
        match key_type {
            crypto::key_types::BLS => {

            },
            crypto::key_types::SECP256K1 => {

            }
        }
    }
}
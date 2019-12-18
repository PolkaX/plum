use crypto::KeyTypeId;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    sync::Arc,
};
use error::Error;

mod address;
mod crypto;
mod keystore;
mod error;

//
// Generate
// Sign
// Remove
//walletNew,
//walletList,
//walletBalance,

/// wallet pointer
pub type WalletPtr = Arc<RwLock<Wallet>>;

pub type Result<T> = std::result::Result<T, Error>;
pub struct Wallet {}

impl Wallet {
        pub fn new_address(key_type: KeyTypeId) -> Result<String> {
            let pair = keystore::KeyPair::generate_key_pair(key_type)?;
            Ok(pair.to_string())
        }
}

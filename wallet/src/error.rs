use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    sync::Arc,
};
use crate::keystore;

/// Keystore error.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// JSON error.
    Json(serde_json::Error),
    /// Invalid password.
    #[display(fmt = "Invalid password")]
    InvalidPassword,
    /// Invalid BIP39 phrase
    #[display(fmt = "Invalid recovery phrase (BIP39) data")]
    InvalidPhrase,
    /// Invalid seed
    #[display(fmt = "Invalid seed")]
    InvalidSeed,
    /// Invalid key type
    #[display(fmt = "Invalid key type")]
    InvalidKeyType,
    /// Keystore unavailable
    #[display(fmt = "Keystore unavailable")]
    Unavailable,
}
/// wallet Result


impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            _ => None,
        }
    }
}
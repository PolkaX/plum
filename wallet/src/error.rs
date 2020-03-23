// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// Type alias to use this library's [`WalletError`] type in a `Result`.
pub type Result<T> = std::result::Result<T, WalletError>;

/// Errors generated from this library.
#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    /// IO error.
    #[error("Invalid password")]
    Io(#[from] std::io::Error),
    /// JSON error.
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    /// Address error.
    #[error("Address error")]
    Address(#[from] plum_address::AddressError),
    /// Crypto error.
    #[error("Crypto error")]
    Crypto(#[from] plum_crypto::CryptoError),
    /// Key type error.
    #[error("{0}")]
    KeyType(#[from] crate::keystore::KeyTypeError),
    /// Key not found.
    #[error("Key not found")]
    KeyNotFound,
    /// Keystore error.
    #[error("Keystore error")]
    KeyStore,
}

impl From<secp256k1::Error> for WalletError {
    fn from(err: secp256k1::Error) -> Self {
        WalletError::Crypto(err.into())
    }
}

impl From<bls::Error> for WalletError {
    fn from(err: bls::Error) -> Self {
        WalletError::Crypto(err.into())
    }
}

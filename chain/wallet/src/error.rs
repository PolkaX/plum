// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_address::AddressError;

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
    /// SECP256k1 error.
    #[error("SECP256K1 error")]
    SECP256K1(#[from] secp256k1::Error),
    /// BLS error.
    #[error("BLS error")]
    BLS,
    /// Address error.
    #[error("Address error")]
    Address(#[from] AddressError),
    /// Key not found.
    #[error("Key not found")]
    KeyNotFound,
    /// Keystore error.
    #[error("Keystore error")]
    KeyStore,
}

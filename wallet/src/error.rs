// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address;
use std::io;
use thiserror;

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
    ///
    #[error("Address error")]
    Address(#[from] address::AddressError),
    /// Invalid password.
    #[error("Invalid password")]
    InvalidPassword,
    ///
    #[error("Invalid Length")]
    InvalidLength,
    /// Keystore is unavailable.
    #[error("Keystore unavailable")]
    Unavailable,
}

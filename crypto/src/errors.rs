// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use thiserror::Error;

/// The error type about crypto.
#[derive(Debug, Eq, PartialEq, Error)]
pub enum CryptoError {
    /// Unknown signature type.
    #[error("Unknown signature type: {0}")]
    UnknownSignatureType(u8),
    /// Secp256k1 error.
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),
    /// BLS error.
    #[error("BLS error: {0}")]
    Bls(String),
}

impl From<bls::Error> for CryptoError {
    fn from(err: bls::Error) -> Self {
        CryptoError::Bls(err.to_string())
    }
}

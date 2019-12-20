use failure;
use std::io;

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
    #[display(fmt = "Invalid Key Type")]
    InvalidKeyType,
    #[display(fmt = "Invalid Signature")]
    InvalidSignature,
    #[display(fmt = "Invalid PublicKey")]
    InvalidPublicKey,
    #[display(fmt = "Invalid SecretKey")]
    InvalidSecretKey,
    #[display(fmt = "Invalid Message")]
    InvalidMessage,
    #[display(fmt = "Invalid Input Length")]
    InvalidInputLength,
    #[display(fmt = "Invalid Length")]
    InvalidLength,
    /// Keystore unavailable
    #[display(fmt = "Keystore unavailable")]
    Unavailable,
}

impl From<std::convert::Infallible> for Error {
    fn from(inf: std::convert::Infallible) -> Self {
        match inf {
            _ => Error::Unavailable,
        }
    }
}

impl From<secp256k1::Error> for Error {
    fn from(e: secp256k1::Error) -> Self {
        match e {
            secp256k1::Error::InvalidSignature => Error::InvalidSignature,
            secp256k1::Error::InvalidPublicKey => Error::InvalidPublicKey,
            secp256k1::Error::InvalidSecretKey => Error::InvalidSecretKey,
            secp256k1::Error::InvalidMessage => Error::InvalidMessage,
            secp256k1::Error::InvalidInputLength => Error::InvalidInputLength,
            _ => Error::Unavailable,
        }
    }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Self {
        match e {
            _ => Error::Unavailable,
        }
    }
}

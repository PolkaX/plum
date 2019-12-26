use data_encoding::{DecodeError, SpecificationError};
use std::io;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown address network")]
    UnknownNetwork,
    #[error("Unknown address protocol")]
    UnknownProtocol,
    #[error("Invalid address payload")]
    InvalidPayload,
    #[error("Invalid address length")]
    InvalidLength,
    #[error("Invalid address checksum")]
    InvalidChecksum,
    #[error("Invalid ID")]
    InvalidID,
    #[error("Invalid KeyType")]
    InvalidKeyType,
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Invalid PublicKey")]
    InvalidPublicKey,
    #[error("Invalid SecretKey")]
    InvalidSecretKey,
    #[error("Invalid Message")]
    InvalidMessage,
    #[error("Invalid Input Length")]
    InvalidInputLength,
    #[error("Data encode error ")]
    Encoding(SpecificationError),
    #[error("Data decode error")]
    DataDecode(DecodeError),
    #[error("Varint U64 decode error")]
    U64Decode(varint::decode::Error),
    #[error("Bytes Convert Failed")]
    BytesConvertFailed,
    #[error("Unavailable")]
    Unavailable,
}

impl From<varint::decode::Error> for Error {
    fn from(e: varint::decode::Error) -> Self {
        Error::U64Decode(e)
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

use data_encoding::{DecodeError, SpecificationError};
use std::io;

#[derive(Debug, derive_more::Display)]
pub enum Error {
    #[display(fmt = "Unknown address network")]
    UnknownNetwork,
    #[display(fmt = "Unknown address protocol")]
    UnknownProtocol,
    #[display(fmt = "Invalid address payload")]
    InvalidPayload,
    #[display(fmt = "Invalid address length")]
    InvalidLength,
    #[display(fmt = "Invalid address checksum")]
    InvalidChecksum,
    #[display(fmt = "Invalid ID")]
    InvalidID,
    #[display(fmt = "Invalid KeyType")]
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
    #[display(fmt = "Data encode error ")]
    Encoding(SpecificationError),
    #[display(fmt = "Data decode error")]
    DataDecode(DecodeError),
    #[display(fmt = "Varint U64 decode error")]
    U64Decode(varint::decode::Error),
    #[display(fmt = "Bytes Convert Failed")]
    BytesConvertFailed,
    #[display(fmt = "Unavailable")]
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

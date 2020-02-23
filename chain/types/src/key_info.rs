// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.
use std::convert::TryFrom;
use thiserror::Error;

use serde_repr::*;

#[derive(Error, Debug)]
pub enum KeyTypeError {
    #[error("unknown type for signature. type: {0}")]
    UnknownType(u8),
}

/// The type of key that store in keystore.
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum KeyType {
    /// SECP256K1 key.
    SECP256K1 = 1,
    /// BLS key.
    BLS = 2,
}

impl TryFrom<u8> for KeyType {
    type Error = KeyTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KeyType::SECP256K1),
            2 => Ok(KeyType::BLS),
            _ => Err(KeyTypeError::UnknownType(value))
        }
    }
}

/// KeyInfo is used for storing keys in KeyStore.
#[derive(Clone, Debug)]
pub struct KeyInfo {
    /// The key type.
    pub ty: KeyType,
    /// The private key corresponding to key type.
    pub privkey: Vec<u8>,
}

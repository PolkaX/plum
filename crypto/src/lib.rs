// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of the general signature.
//! In addition, the simple public key wrapper and private key wrapper are implemented.

#![deny(missing_docs)]

mod errors;
mod key; // just a simple wrapper for public key and private key.
mod signature;

pub use self::errors::CryptoError;
pub use self::key::{PrivateKey, PublicKey};
pub use self::signature::serde::cbor as signature_cbor;
pub use self::signature::{Signature, SignatureType, SIGNATURE_MAX_LENGTH};

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of the general signature, with CBOR and JSON serialization/deserialization.
//! In addition, the simple public key wrapper and private key wrapper are implemented.

#![deny(missing_docs)]

extern crate bls_signatures as bls;

mod errors;
mod key; // just a simple wrapper for public key and private key.
mod randomness;
mod signature;
mod vrf;

pub use self::errors::CryptoError;
pub use self::key::{PrivateKey, PublicKey};
pub use self::randomness::DomainSeparationTag;
pub use self::signature::{Signature, SignatureType, SIGNATURE_MAX_LENGTH};
pub use self::vrf::{compute_vrf, verify_vrf, VrfPrivateKey, VrfProof, VrfPublicKey};

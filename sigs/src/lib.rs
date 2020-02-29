mod bls;
mod secp256k1;
mod signer;

use anyhow::Result;

pub use crate::bls::bls_sign;
pub use crate::secp256k1::secp256k1_sign;

pub enum SignatureType {
    BLS,
    SECP256K1,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeyType {
    ///
    SECP256K1,
    ///
    BLS,
}

/// KeyInfo is used for storing keys in KeyStore.
#[derive(Clone, Debug)]
pub struct KeyInfo {
    /// The key type.
    pub ty: KeyType,
    /// The private key corresponding to key type.
    pub privkey: Vec<u8>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Signature {
    ///
    pub ty: KeyType,
    ///
    pub data: Vec<u8>,
}

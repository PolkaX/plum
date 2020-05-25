// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::errors::CryptoError;

/// The general public key.
#[derive(Debug, Clone, PartialEq)]
pub enum PublicKey {
    /// `Secp256k1` public key
    Secp256k1(secp256k1::PublicKey),
    /// `BLS` public key
    Bls(bls::PublicKey),
}

impl PublicKey {
    /// Create a `secp256k1` public key with the given bytes.
    pub fn new_secp256k1<K: AsRef<[u8]>>(pubkey: K) -> Result<Self, CryptoError> {
        let pubkey = secp256k1::PublicKey::parse_slice(pubkey.as_ref(), None)?;
        Ok(PublicKey::Secp256k1(pubkey))
    }

    /// Create a `BLS` public key with the given bytes.
    pub fn new_bls<K: AsRef<[u8]>>(pubkey: K) -> Result<Self, CryptoError> {
        use bls::Serialize;
        let pubkey = bls::PublicKey::from_bytes(pubkey.as_ref())?;
        Ok(PublicKey::Bls(pubkey))
    }

    /// Get the public key from the private key.
    pub fn from_privkey(privkey: &PrivateKey) -> Self {
        match privkey {
            PrivateKey::Secp256k1(privkey) => {
                let pubkey = secp256k1::PublicKey::from_secret_key(&privkey);
                PublicKey::Secp256k1(pubkey)
            }
            PrivateKey::Bls(privkey) => {
                let pubkey = privkey.public_key();
                PublicKey::Bls(pubkey)
            }
        }
    }

    /// Convert the public key into bytes.
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            PublicKey::Secp256k1(pubkey) => pubkey.serialize().to_vec(),
            PublicKey::Bls(pubkey) => {
                use bls::Serialize;
                pubkey.as_bytes()
            }
        }
    }
}

/// The general private key.
#[derive(Debug, Clone, PartialEq)]
pub enum PrivateKey {
    /// `Secp256k1` private key
    Secp256k1(secp256k1::SecretKey),
    /// `BLS` private key
    Bls(bls::PrivateKey),
}

impl PrivateKey {
    /// Create a `secp256k1` private key with the given bytes.
    pub fn new_secp256k1<K: AsRef<[u8]>>(privkey: K) -> Result<Self, CryptoError> {
        Ok(PrivateKey::Secp256k1(secp256k1::SecretKey::parse_slice(
            privkey.as_ref(),
        )?))
    }

    /// Create a `bls` private key with the given bytes.
    pub fn new_bls<K: AsRef<[u8]>>(privkey: K) -> Result<Self, CryptoError> {
        use bls::Serialize;
        Ok(PrivateKey::Bls(bls::PrivateKey::from_bytes(
            privkey.as_ref(),
        )?))
    }

    /// Generate a `secp256k1` private key randomly.
    ///
    /// Returns the private key.
    pub fn generate_secp256k1_privkey() -> Self {
        PrivateKey::Secp256k1(secp256k1::SecretKey::random(&mut rand::rngs::OsRng))
    }

    /// Generate a `bls` private key randomly.
    ///
    /// Returns the private key.
    pub fn generate_bls_privkey() -> Self {
        PrivateKey::Bls(bls::PrivateKey::generate(&mut rand::rngs::OsRng))
    }

    /// Convert the private key into bytes.
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            PrivateKey::Secp256k1(privkey) => privkey.serialize().to_vec(),
            PrivateKey::Bls(privkey) => {
                use bls::Serialize;
                privkey.as_bytes()
            }
        }
    }
}

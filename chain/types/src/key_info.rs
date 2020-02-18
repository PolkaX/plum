// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

/// The type of key that store in keystore.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    /// SECP256K1 key.
    SECP256K1,
    /// BLS key.
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

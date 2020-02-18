// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::key_info::KeyInfo;

/// KeyStore is used for storing secret keys.
pub trait KeyStore {
    /// The error type that keystore operations may return.
    type Error;

    /// List lists all the keys stored in the KeyStore.
    fn list(&self) -> Result<Vec<String>, Self::Error>;
    /// Get gets a key out of keystore and returns KeyInfo corresponding to named key.
    fn get(&self, _: &str) -> Result<KeyInfo, Self::Error>;
    /// Put saves a key info under given name.
    fn put(&mut self, _: String, _: KeyInfo) -> Result<(), Self::Error>;
    /// Delete removes a key from keystore.
    fn delete(&mut self, _: String) -> Result<(), Self::Error>;
}

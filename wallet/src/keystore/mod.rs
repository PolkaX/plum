// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod key_info;
mod memory;

pub use self::key_info::{key_info_json, key_type_json, KeyInfo, KeyType};
pub use self::memory::MemKeyStore;

/// The default keystore path.
pub const DEFAULT_KEYSTORE_PATH: &str = "/.plum/keystore/";

/// KeyStore is used for operating key info.
pub trait KeyStore {
    /// The KeyStore error.
    type Error: std::fmt::Display;

    /// Lists all the keys stored in the KeyStore.
    fn list(&self) -> Result<Vec<String>, Self::Error>;

    /// Gets a key out of the KeyStore.
    ///
    /// If the KeyStore did not have this key present, None is returned.
    /// If the KeyStore did have this key present, the Some(&KeyInfo) is returned.
    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<KeyInfo>, Self::Error>;

    /// Saves a key info under given name.
    fn put(&mut self, key: String, info: KeyInfo) -> Result<(), Self::Error>;

    /// Removes a KeyInfo from the KeyStore corresponding to the named key.
    fn delete<K: AsRef<str>>(&mut self, key: K) -> Result<(), Self::Error>;
}

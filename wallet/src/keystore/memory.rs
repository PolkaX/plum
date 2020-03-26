// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;

use crate::keystore::{KeyInfo, KeyStore};

/// A example KeyStore that stores all key info in the memory.
#[derive(Default)]
pub struct MemKeyStore {
    map: HashMap<String, KeyInfo>,
}

impl MemKeyStore {
    /// Create a new Memory KeyStore.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl KeyStore for MemKeyStore {
    type Error = String;

    fn list(&self) -> Result<Vec<String>, Self::Error> {
        Ok(self.map.keys().cloned().collect())
    }

    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<KeyInfo>, Self::Error> {
        Ok(self.map.get(key.as_ref()).cloned())
    }

    fn put(&mut self, key: String, info: KeyInfo) -> Result<(), Self::Error> {
        let _ = self.map.insert(key, info);
        Ok(())
    }

    fn delete<K: AsRef<str>>(&mut self, key: K) -> Result<(), Self::Error> {
        let _ = self.map.remove(key.as_ref());
        Ok(())
    }
}

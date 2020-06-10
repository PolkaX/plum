// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::collections::HashMap;

use crate::error::{DataStoreError, Result};
use crate::key::Key;
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};

/// MapDataStore use HashMap for internal storage.
#[derive(Clone, Debug, Default)]
pub struct MapDataStore {
    values: HashMap<Key, Vec<u8>>,
}

impl MapDataStore {
    /// Create a new MapDataStore.
    pub fn new() -> Self {
        Self::default()
    }
}

impl DataStore for MapDataStore {
    fn sync<K>(&self, _prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        Ok(())
    }

    fn close(&self) -> Result<()> {
        Ok(())
    }
}

impl DataStoreRead for MapDataStore {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        Ok(self
            .values
            .get(key)
            .ok_or(DataStoreError::NotFound(key.to_string()))?
            .to_owned())
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        Ok(self.values.contains_key(key.borrow()))
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        Ok(self
            .values
            .get(key)
            .ok_or(DataStoreError::NotFound(key.to_string()))?
            .len())
    }
}

impl DataStoreWrite for MapDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.values.insert(key.into(), value.into());
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.values.remove(key.borrow());
        Ok(())
    }
}

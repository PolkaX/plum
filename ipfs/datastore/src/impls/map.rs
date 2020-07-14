// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Result;

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
    fn sync<K>(&mut self, _prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

impl DataStoreRead for MapDataStore {
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        Ok(self.values.get(key.borrow()).cloned())
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        Ok(self.values.contains_key(key.borrow()))
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

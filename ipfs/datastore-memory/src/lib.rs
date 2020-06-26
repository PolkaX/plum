// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! A thread-safe datastore living in memory.

#![deny(missing_docs)]

use std::borrow::Borrow;

use ipfs_datastore::{
    DataStore, DataStoreError, DataStoreRead, DataStoreWrite, Key, MapDataStore, SyncDataStore,
};

pub(crate) type Result<T> = std::result::Result<T, DataStoreError>;

/// A thread-safe datastore living in memory, which is generally intended for tests.
#[derive(Clone)]
pub struct MemoryDataStore {
    datastore: SyncDataStore<MapDataStore>,
}

impl DataStore for MemoryDataStore {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl DataStoreRead for MemoryDataStore {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.size(key)
    }
}

impl DataStoreWrite for MemoryDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.delete(key)
    }
}

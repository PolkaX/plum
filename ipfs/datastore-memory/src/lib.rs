// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! A thread-safe datastore living in memory.

#![deny(missing_docs)]

use std::borrow::Borrow;
use std::io;

use ipfs_datastore::{DataStore, DataStoreRead, DataStoreWrite};
use ipfs_datastore::{Key, MapDataStore, SyncDataStore};

/// A thread-safe datastore living in memory, which is generally intended for tests.
#[derive(Clone)]
pub struct MemoryDataStore {
    datastore: SyncDataStore<MapDataStore>,
}

impl DataStore for MemoryDataStore {
    fn sync<K>(&mut self, prefix: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> io::Result<()> {
        self.datastore.close()
    }
}

impl DataStoreRead for MemoryDataStore {
    fn get<K>(&self, key: &K) -> io::Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> io::Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.has(key)
    }
}

impl DataStoreWrite for MemoryDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.delete(key)
    }
}

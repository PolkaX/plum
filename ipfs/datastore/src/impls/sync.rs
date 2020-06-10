// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::error::Result;
use crate::key::Key;
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};

/// SyncDataStore contains a child datastore wrapper using mutex.
pub struct SyncDataStore<DS: DataStore> {
    child: Arc<Mutex<DS>>,
}

impl<DS: DataStore> SyncDataStore<DS> {
    /// Create a new datastore with a coarse lock around the entire datastore,
    /// for every single operation.
    pub fn new(datastore: DS) -> Self {
        Self {
            child: Arc::new(Mutex::new(datastore)),
        }
    }
}

impl<DS: DataStore> DataStore for SyncDataStore<DS> {
    fn sync<K>(&self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        self.child.lock().sync(prefix)
    }

    fn close(&self) -> Result<()> {
        self.child.lock().close()
    }
}

impl<DS: DataStore> DataStoreRead for SyncDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.child.lock().get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.child.lock().has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.child.lock().size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for SyncDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.child.lock().put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.child.lock().delete(key)
    }
}

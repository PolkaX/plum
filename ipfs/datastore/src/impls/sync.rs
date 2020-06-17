// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Batch, BatchDataStore};
use crate::store::{Batching, DataStore, DataStoreRead, DataStoreWrite};

/// SyncDataStore contains a datastore wrapper using mutex.
pub struct SyncDataStore<DS: DataStore> {
    datastore: Arc<Mutex<DS>>,
}

impl<DS: DataStore> SyncDataStore<DS> {
    /// Create a new datastore with a coarse lock around the entire datastore,
    /// for every single operation.
    pub fn new(datastore: DS) -> Self {
        Self {
            datastore: Arc::new(Mutex::new(datastore)),
        }
    }
}

impl<DS: DataStore> DataStore for SyncDataStore<DS> {
    fn sync<K>(&mut self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        self.datastore.lock().sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.lock().close()
    }
}

impl<DS: DataStore> DataStoreRead for SyncDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for SyncDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.lock().put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().delete(key)
    }
}

impl<BDS: BatchDataStore> Batching for SyncDataStore<BDS> {
    type Batch = SyncBatchDataStore<BDS>;

    fn batch(self) -> Result<Self::Batch> {
        Ok(SyncBatchDataStore {
            datastore: self.datastore,
        })
    }
}

// ============================================================================

/// SyncDataStore contains a datastore wrapper using mutex.
pub struct SyncBatchDataStore<BDS: BatchDataStore> {
    datastore: Arc<Mutex<BDS>>,
}

impl<BDS: BatchDataStore> SyncBatchDataStore<BDS> {
    /// Create a new datastore with a coarse lock around the entire datastore,
    /// for batching operations.
    pub fn new(datastore: BDS) -> Self {
        Self {
            datastore: Arc::new(Mutex::new(datastore)),
        }
    }
}

impl<BDS: BatchDataStore> DataStore for SyncBatchDataStore<BDS> {
    fn sync<K>(&mut self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        self.datastore.lock().sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.lock().close()
    }
}

impl<BDS: BatchDataStore> DataStoreRead for SyncBatchDataStore<BDS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().size(key)
    }
}

impl<BDS: BatchDataStore> DataStoreWrite for SyncBatchDataStore<BDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.lock().put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.lock().delete(key)
    }
}

impl<BDS: BatchDataStore> Batch for SyncBatchDataStore<BDS> {
    fn commit(&mut self) -> Result<()> {
        self.datastore.lock().commit()
    }
}

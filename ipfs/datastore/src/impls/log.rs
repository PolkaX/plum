// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use log::info;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Batch, BatchDataStore};
use crate::store::{Check, CheckedDataStore};
use crate::store::{DataStore, DataStoreRead, DataStoreWrite, ToBatch};
use crate::store::{Gc, GcDataStore};
use crate::store::{Persistent, PersistentDataStore};
use crate::store::{Scrub, ScrubbedDataStore};

/// LogDataStore logs all accesses through the datastore.
#[derive(Clone, Debug)]
pub struct LogDataStore<DS: DataStore> {
    name: String,
    datastore: DS,
}

impl<DS: DataStore> LogDataStore<DS> {
    /// Create a new LogDataStore.
    pub fn new<S: Into<String>>(name: S, datastore: DS) -> Self {
        Self {
            name: name.into(),
            datastore,
        }
    }
}

impl<DS: DataStore> DataStore for LogDataStore<DS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        info!("{}: sync {}", self.name, prefix.borrow());
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        info!("{}: close", self.name);
        self.datastore.close()
    }
}

impl<DS: DataStore> DataStoreRead for LogDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        info!("{}: get {}", self.name, key.borrow());
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        info!("{}: has {}", self.name, key.borrow());
        self.datastore.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        info!("{}: size {}", self.name, key.borrow());
        self.datastore.size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for LogDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = key.into();
        let value = value.into();
        info!("{}: put {} - {:?}", self.name, key, value);
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        info!("{}: delete {}", self.name, key.borrow());
        self.datastore.delete(key)
    }
}

impl<DS: CheckedDataStore> Check for LogDataStore<DS> {
    fn check(&self) -> Result<()> {
        info!("{}: check", self.name);
        self.datastore.check()
    }
}

impl<DS: GcDataStore> Gc for LogDataStore<DS> {
    fn collect_garbage(&self) -> Result<()> {
        info!("{}: collect_garbage", self.name);
        self.datastore.collect_garbage()
    }
}

impl<DS: PersistentDataStore> Persistent for LogDataStore<DS> {
    fn disk_usage(&self) -> Result<u64> {
        info!("{}: disk_usage", self.name);
        self.datastore.disk_usage()
    }
}

impl<DS: ScrubbedDataStore> Scrub for LogDataStore<DS> {
    fn scrub(&self) -> Result<()> {
        info!("{}: scrub", self.name);
        self.datastore.scrub()
    }
}

impl<BDS: BatchDataStore> ToBatch for LogDataStore<BDS> {
    type Batch = LogBatchDataStore<BDS>;

    fn batch(self) -> Result<Self::Batch> {
        info!("{}: batch", self.name);
        Ok(LogBatchDataStore::new(self.name, self.datastore))
    }
}

// ============================================================================

/// LogBatchDataStore logs all accesses through the batching data store.
pub struct LogBatchDataStore<BDS: BatchDataStore> {
    name: String,
    datastore: BDS,
}

impl<BDS: BatchDataStore> LogBatchDataStore<BDS> {
    /// Create a new LogBatchDataStore.
    pub fn new<S: Into<String>>(name: S, datastore: BDS) -> Self {
        Self {
            name: name.into(),
            datastore,
        }
    }
}

impl<BDS: BatchDataStore> DataStore for LogBatchDataStore<BDS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        info!("{}: batch sync {}", self.name, prefix.borrow());
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        info!("{}: batch close", self.name);
        self.datastore.close()
    }
}

impl<BDS: BatchDataStore> DataStoreRead for LogBatchDataStore<BDS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        info!("{}: batch get {}", self.name, key.borrow());
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        info!("{}: batch has {}", self.name, key.borrow());
        self.datastore.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        info!("{}: batch size {}", self.name, key.borrow());
        self.datastore.size(key)
    }
}

impl<BDS: BatchDataStore> DataStoreWrite for LogBatchDataStore<BDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = key.into();
        let value = value.into();
        info!("{}: batch put {} - {:?}", self.name, key, value);
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        info!("{}: batch delete {}", self.name, key.borrow());
        self.datastore.delete(key)
    }
}

impl<BDS: BatchDataStore> Batch for LogBatchDataStore<BDS> {
    fn commit(&mut self) -> Result<()> {
        info!("{}: batch commit", self.name);
        self.commit()
    }
}

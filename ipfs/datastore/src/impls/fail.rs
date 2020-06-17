// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Batch, BatchDataStore};
use crate::store::{DataStore, DataStoreRead, DataStoreWrite, ToBatch};
use crate::store::{Persistent, PersistentDataStore};

/// The user-provided fail function.
pub trait FailFn: Fn(&str) -> Result<()> {}

/// FailDataStore is a datastore which fails according to a user-provided function.
pub struct FailDataStore<F: FailFn, DS: DataStore> {
    fail_fn: F,
    datastore: DS,
}

impl<F: FailFn, DS: DataStore> FailDataStore<F, DS> {
    /// Create a new datastore with the given error function.
    /// The `fail_fn` will be called with different strings depending on the datastore function.
    pub fn new(fail_fn: F, datastore: DS) -> Self {
        Self { fail_fn, datastore }
    }
}

impl<F: FailFn, DS: DataStore> DataStore for FailDataStore<F, DS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("sync")?;
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl<F: FailFn, DS: DataStore> DataStoreRead for FailDataStore<F, DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("put")?;
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("has")?;
        self.datastore.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("size")?;
        self.datastore.size(key)
    }
}

impl<F: FailFn, DS: DataStore> DataStoreWrite for FailDataStore<F, DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        (self.fail_fn)("put")?;
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("delete")?;
        self.datastore.delete(key)
    }
}

impl<F: FailFn, DS: PersistentDataStore> Persistent for FailDataStore<F, DS> {
    fn disk_usage(&self) -> Result<u64> {
        (self.fail_fn)("disk-usage")?;
        self.datastore.disk_usage()
    }
}

impl<F: FailFn, BDS: BatchDataStore> ToBatch for FailDataStore<F, BDS> {
    type Batch = FailBatchDataStore<F, BDS>;

    fn batch(self) -> Result<Self::Batch> {
        Ok(FailBatchDataStore::new(self.fail_fn, self.datastore))
    }
}

// ============================================================================

/// FailBatchDataStore implements batching operations on the FailDataStore.
pub struct FailBatchDataStore<F: FailFn, BDS: BatchDataStore> {
    fail_fn: F,
    datastore: BDS,
}

impl<F: FailFn, BDS: BatchDataStore> FailBatchDataStore<F, BDS> {
    /// Create a new batching datastore with the given error function.
    /// The `fail_fn` will be called with different strings depending on the datastore function.
    pub fn new(fail_fn: F, datastore: BDS) -> Self {
        Self { fail_fn, datastore }
    }
}

impl<F: FailFn, BDS: BatchDataStore> DataStore for FailBatchDataStore<F, BDS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("batch-sync")?;
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl<F: FailFn, BDS: BatchDataStore> DataStoreRead for FailBatchDataStore<F, BDS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("batch-put")?;
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("batch-has")?;
        self.datastore.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("batch-size")?;
        self.datastore.size(key)
    }
}

impl<F: FailFn, BDS: BatchDataStore> DataStoreWrite for FailBatchDataStore<F, BDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        (self.fail_fn)("batch-put")?;
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("batch-delete")?;
        self.datastore.delete(key)
    }
}

impl<F: FailFn, BDS: BatchDataStore> Batch for FailBatchDataStore<F, BDS> {
    fn commit(&mut self) -> Result<()> {
        (self.fail_fn)("batch-commit")?;
        self.datastore.commit()
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::io::Result;

use crate::key::Key;
use crate::store::{BatchDataStore, ToBatch, ToTxn, TxnDataStore};
use crate::store::{Check, CheckedBatchDataStore, CheckedDataStore, CheckedTxnDataStore};
use crate::store::{DataStore, DataStoreBatch, DataStoreRead, DataStoreTxn, DataStoreWrite};
use crate::store::{Gc, GcBatchDataStore, GcDataStore, GcTxnDataStore};
use crate::store::{
    Persistent, PersistentBatchDataStore, PersistentDataStore, PersistentTxnDataStore,
};
use crate::store::{Scrub, ScrubbedBatchDataStore, ScrubbedDataStore, ScrubbedTxnDataStore};

/// The user-provided fail function.
pub trait FailFn: Fn(&str) -> Result<()> + Clone + Sync + Send + 'static {}

/// FailDataStore is a datastore which fails according to a user-provided function.
#[derive(Clone)]
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
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
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

impl<F: FailFn, DS: CheckedDataStore> Check for FailDataStore<F, DS> {
    fn check(&self) -> Result<()> {
        (self.fail_fn)("check")?;
        self.datastore.check()
    }
}

impl<F: FailFn, DS: GcDataStore> Gc for FailDataStore<F, DS> {
    fn collect_garbage(&self) -> Result<()> {
        (self.fail_fn)("collect-garbage")?;
        self.datastore.collect_garbage()
    }
}

impl<F: FailFn, DS: PersistentDataStore> Persistent for FailDataStore<F, DS> {
    fn disk_usage(&self) -> Result<u64> {
        (self.fail_fn)("disk-usage")?;
        self.datastore.disk_usage()
    }
}

impl<F: FailFn, DS: ScrubbedDataStore> Scrub for FailDataStore<F, DS> {
    fn scrub(&self) -> Result<()> {
        (self.fail_fn)("scrub")?;
        self.datastore.scrub()
    }
}

impl<F: FailFn, BDS: BatchDataStore> ToBatch for FailDataStore<F, BDS> {
    type Batch = FailBatchDataStore<F, BDS>;

    fn batch(&self) -> Result<Self::Batch> {
        Ok(FailBatchDataStore::new(
            self.fail_fn.clone(),
            self.datastore.clone(),
        ))
    }
}

impl<F: FailFn, TDS: TxnDataStore> ToTxn for FailDataStore<F, TDS> {
    type Txn = FailTxnDataStore<F, TDS>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(FailTxnDataStore::new(
            self.fail_fn.clone(),
            self.datastore.clone(),
        ))
    }
}

// ============================================================================

/// FailBatchDataStore implements batching operations on the FailDataStore.
#[derive(Clone)]
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
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
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

impl<F: FailFn, BDS: BatchDataStore> DataStoreBatch for FailBatchDataStore<F, BDS> {
    fn commit(&mut self) -> Result<()> {
        (self.fail_fn)("batch-commit")?;
        self.datastore.commit()
    }
}

impl<F: FailFn, BDS: CheckedBatchDataStore> Check for FailBatchDataStore<F, BDS> {
    fn check(&self) -> Result<()> {
        (self.fail_fn)("check")?;
        self.datastore.check()
    }
}

impl<F: FailFn, BDS: GcBatchDataStore> Gc for FailBatchDataStore<F, BDS> {
    fn collect_garbage(&self) -> Result<()> {
        (self.fail_fn)("collect-garbage")?;
        self.datastore.collect_garbage()
    }
}

impl<F: FailFn, BDS: PersistentBatchDataStore> Persistent for FailBatchDataStore<F, BDS> {
    fn disk_usage(&self) -> Result<u64> {
        (self.fail_fn)("disk-usage")?;
        self.datastore.disk_usage()
    }
}

impl<F: FailFn, BDS: ScrubbedBatchDataStore> Scrub for FailBatchDataStore<F, BDS> {
    fn scrub(&self) -> Result<()> {
        (self.fail_fn)("scrub")?;
        self.datastore.scrub()
    }
}

impl<F: FailFn, TDS: TxnDataStore> ToTxn for FailBatchDataStore<F, TDS> {
    type Txn = FailTxnDataStore<F, TDS>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(FailTxnDataStore::new(
            self.fail_fn.clone(),
            self.datastore.clone(),
        ))
    }
}

// ============================================================================

/// FailTxnDataStore implements transaction operations on the FailDataStore.
#[derive(Clone)]
pub struct FailTxnDataStore<F: FailFn, TDS: TxnDataStore> {
    fail_fn: F,
    datastore: TDS,
}

impl<F: FailFn, TDS: TxnDataStore> FailTxnDataStore<F, TDS> {
    /// Create a new transaction datastore with the given error function.
    /// The `fail_fn` will be called with different strings depending on the datastore function.
    pub fn new(fail_fn: F, datastore: TDS) -> Self {
        Self { fail_fn, datastore }
    }
}

impl<F: FailFn, TDS: TxnDataStore> DataStore for FailTxnDataStore<F, TDS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("txn-sync")?;
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl<F: FailFn, TDS: TxnDataStore> DataStoreRead for FailTxnDataStore<F, TDS> {
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("txn-put")?;
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("txn-has")?;
        self.datastore.has(key)
    }
}

impl<F: FailFn, TDS: TxnDataStore> DataStoreWrite for FailTxnDataStore<F, TDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        (self.fail_fn)("txn-put")?;
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.fail_fn)("txn-delete")?;
        self.datastore.delete(key)
    }
}

impl<F: FailFn, TDS: TxnDataStore> DataStoreBatch for FailTxnDataStore<F, TDS> {
    fn commit(&mut self) -> Result<()> {
        (self.fail_fn)("txn-commit")?;
        self.datastore.commit()
    }
}

impl<F: FailFn, TDS: TxnDataStore> DataStoreTxn for FailTxnDataStore<F, TDS> {
    fn discard(&mut self) -> Result<()> {
        (self.fail_fn)("txn-discard")?;
        self.datastore.discard()
    }
}

impl<F: FailFn, TDS: CheckedTxnDataStore> Check for FailTxnDataStore<F, TDS> {
    fn check(&self) -> Result<()> {
        (self.fail_fn)("check")?;
        self.datastore.check()
    }
}

impl<F: FailFn, TDS: GcTxnDataStore> Gc for FailTxnDataStore<F, TDS> {
    fn collect_garbage(&self) -> Result<()> {
        (self.fail_fn)("collect-garbage")?;
        self.datastore.collect_garbage()
    }
}

impl<F: FailFn, TDS: PersistentTxnDataStore> Persistent for FailTxnDataStore<F, TDS> {
    fn disk_usage(&self) -> Result<u64> {
        (self.fail_fn)("disk-usage")?;
        self.datastore.disk_usage()
    }
}

impl<F: FailFn, TDS: ScrubbedTxnDataStore> Scrub for FailTxnDataStore<F, TDS> {
    fn scrub(&self) -> Result<()> {
        (self.fail_fn)("scrub")?;
        self.datastore.scrub()
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::io::Result;

use crate::impls::{BasicBatchDataStore, BasicTxnDataStore};
use crate::key::Key;
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};
use crate::store::{Persistent, PersistentDataStore};
use crate::store::{ToBatch, ToTxn};

/// The delay interface for delay operation.
pub trait Delay: Clone + Sync + Send + 'static {
    /// Wait for a period of time duration util time out before applying the operation.
    fn wait(&self);
}

/// DelayDataStore is an adapter that delays operations on the inner datastore.
#[derive(Clone)]
pub struct DelayDataStore<DL: Delay, DS: DataStore> {
    delay: DL,
    datastore: DS,
}

impl<DL: Delay, DS: DataStore> DelayDataStore<DL, DS> {
    /// Create a new DelayDataStore.
    pub fn new(delay: DL, datastore: DS) -> Self {
        Self { delay, datastore }
    }
}

impl<DL: Delay, DS: DataStore> DataStore for DelayDataStore<DL, DS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.datastore.sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl<DL: Delay, DS: DataStore> DataStoreRead for DelayDataStore<DL, DS> {
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.datastore.has(key)
    }
}

impl<DL: Delay, DS: DataStore> DataStoreWrite for DelayDataStore<DL, DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.delay.wait();
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.datastore.delete(key)
    }
}

impl<DL: Delay, DS: PersistentDataStore> Persistent for DelayDataStore<DL, DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.delay.wait();
        self.datastore.disk_usage()
    }
}

impl<DL: Delay, DS: DataStore> ToBatch for DelayDataStore<DL, DS> {
    type Batch = BasicBatchDataStore<DelayDataStore<DL, DS>>;

    fn batch(&self) -> Result<Self::Batch> {
        Ok(BasicBatchDataStore::new(self.clone()))
    }
}

impl<DL: Delay, DS: DataStore> ToTxn for DelayDataStore<DL, DS> {
    type Txn = BasicTxnDataStore<DelayDataStore<DL, DS>>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(BasicTxnDataStore::new(self.clone()))
    }
}

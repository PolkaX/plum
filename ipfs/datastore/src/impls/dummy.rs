// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::io::Result;

use crate::impls::{BasicBatchDataStore, BasicTxnDataStore};
use crate::key::Key;
use crate::store::{Check, Gc, Persistent, Scrub};
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};
use crate::store::{ToBatch, ToTxn};

/// DummyDataStore stores nothing, but conforms to the API.
/// Useful to test with.
#[derive(Copy, Clone)]
pub struct DummyDataStore;

impl DataStore for DummyDataStore {
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

impl DataStoreRead for DummyDataStore {
    fn get<K>(&self, _key: &K) -> Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        Ok(None)
    }

    fn has<K>(&self, _key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        Ok(false)
    }
}

impl DataStoreWrite for DummyDataStore {
    fn put<K, V>(&mut self, _key: K, _value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        Ok(())
    }

    fn delete<K>(&mut self, _key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        Ok(())
    }
}

impl Check for DummyDataStore {
    fn check(&self) -> Result<()> {
        Ok(())
    }
}

impl Gc for DummyDataStore {
    fn collect_garbage(&self) -> Result<()> {
        Ok(())
    }
}

impl Persistent for DummyDataStore {
    fn disk_usage(&self) -> Result<u64> {
        Ok(0)
    }
}

impl Scrub for DummyDataStore {
    fn scrub(&self) -> Result<()> {
        Ok(())
    }
}

impl ToBatch for DummyDataStore {
    type Batch = BasicBatchDataStore<DummyDataStore>;

    fn batch(&self) -> Result<Self::Batch> {
        Ok(BasicBatchDataStore::new(DummyDataStore))
    }
}

impl ToTxn for DummyDataStore {
    type Txn = BasicTxnDataStore<DummyDataStore>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(BasicTxnDataStore::new(DummyDataStore))
    }
}

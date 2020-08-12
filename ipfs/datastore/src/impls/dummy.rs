// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::io;

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
    fn sync<K>(&mut self, _prefix: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        Ok(())
    }

    fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl DataStoreRead for DummyDataStore {
    fn get<K>(&self, _key: &K) -> io::Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        Ok(None)
    }

    fn has<K>(&self, _key: &K) -> io::Result<bool>
    where
        K: Borrow<Key>,
    {
        Ok(false)
    }
}

impl DataStoreWrite for DummyDataStore {
    fn put<K, V>(&mut self, _key: K, _value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        Ok(())
    }

    fn delete<K>(&mut self, _key: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        Ok(())
    }
}

impl Check for DummyDataStore {
    fn check(&self) -> io::Result<()> {
        Ok(())
    }
}

impl Gc for DummyDataStore {
    fn collect_garbage(&self) -> io::Result<()> {
        Ok(())
    }
}

impl Persistent for DummyDataStore {
    fn disk_usage(&self) -> io::Result<u64> {
        Ok(0)
    }
}

impl Scrub for DummyDataStore {
    fn scrub(&self) -> io::Result<()> {
        Ok(())
    }
}

impl ToBatch for DummyDataStore {
    type Batch = BasicBatchDataStore<DummyDataStore>;

    fn batch(&self) -> io::Result<Self::Batch> {
        Ok(BasicBatchDataStore::new(DummyDataStore))
    }
}

impl ToTxn for DummyDataStore {
    type Txn = BasicTxnDataStore<DummyDataStore>;

    fn txn(&self, _read_only: bool) -> io::Result<Self::Txn> {
        Ok(BasicTxnDataStore::new(DummyDataStore))
    }
}

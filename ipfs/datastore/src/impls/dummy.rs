// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use crate::error::{DataStoreError, Result};
use crate::impls::BasicBatchDataStore;
use crate::key::Key;
use crate::store::{Batching, DataStore, DataStoreRead, DataStoreWrite};

/// DummyDataStore stores nothing, but conforms to the API.
/// Useful to test with.
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
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        Err(DataStoreError::NotFound(key.borrow().to_string()))
    }

    fn has<K>(&self, _key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        Ok(false)
    }

    fn size<K>(&self, _key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        Ok(0)
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

impl Batching for DummyDataStore {
    type Batch = BasicBatchDataStore<DummyDataStore>;

    fn batch(self) -> Result<Self::Batch> {
        Ok(BasicBatchDataStore::new(self))
    }
}

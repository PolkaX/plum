// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::collections::HashMap;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Batch, DataStore, DataStoreRead, DataStoreWrite};

enum BatchOp {
    Put(Vec<u8>), // a single put operation of batched operations
    Delete,       // a single delete operation of batched operations.
}

/// BasicBatchDataStore implements the transaction interface for data stores
/// who do not have any sort of underlying transactional support.
pub struct BasicBatchDataStore<DS: DataStore> {
    ops: HashMap<Key, BatchOp>,
    datastore: DS,
}

impl<DS: DataStore> BasicBatchDataStore<DS> {
    /// Create a new basic batching datastore.
    pub fn new(datastore: DS) -> Self {
        Self {
            ops: HashMap::new(),
            datastore,
        }
    }
}

impl<DS: DataStore> DataStoreRead for BasicBatchDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for BasicBatchDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.ops.insert(key.into(), BatchOp::Put(value.into()));
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.ops.insert(key.borrow().to_owned(), BatchOp::Delete);
        Ok(())
    }
}

impl<DS: DataStore> Batch for BasicBatchDataStore<DS> {
    fn commit(&mut self) -> Result<()> {
        for (key, op) in &self.ops {
            match op {
                BatchOp::Put(value) => self.datastore.put(key, value.to_owned())?,
                BatchOp::Delete => self.datastore.delete(&key)?,
            }
        }
        self.ops.clear();
        Ok(())
    }
}

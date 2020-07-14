// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Result;

use crate::key::Key;
use crate::store::ToTxn;
use crate::store::{Check, CheckedDataStore};
use crate::store::{DataStore, DataStoreBatch, DataStoreRead, DataStoreTxn, DataStoreWrite};
use crate::store::{Gc, GcDataStore};
use crate::store::{Persistent, PersistentDataStore};
use crate::store::{Scrub, ScrubbedDataStore};

#[derive(Clone)]
enum Op {
    Put(Vec<u8>), // a single put operation of batched operations
    Delete,       // a single delete operation of batched operations.
}

/// BasicBatchDataStore implements the batch interface for data stores
/// who do not have any sort of underlying batch support.
#[derive(Clone)]
pub struct BasicBatchDataStore<DS: DataStore> {
    ops: HashMap<Key, Op>,
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
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
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
}

impl<DS: DataStore> DataStoreWrite for BasicBatchDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.ops.insert(key.into(), Op::Put(value.into()));
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.ops.insert(key.borrow().to_owned(), Op::Delete);
        Ok(())
    }
}

impl<DS: DataStore> DataStoreBatch for BasicBatchDataStore<DS> {
    fn commit(&mut self) -> Result<()> {
        for (key, op) in &self.ops {
            match op {
                Op::Put(value) => self.datastore.put(key, value.to_owned())?,
                Op::Delete => self.datastore.delete(&key)?,
            }
        }
        self.ops.clear();
        Ok(())
    }
}

impl<DS: CheckedDataStore> Check for BasicBatchDataStore<DS> {
    fn check(&self) -> Result<()> {
        self.datastore.check()
    }
}

impl<DS: GcDataStore> Gc for BasicBatchDataStore<DS> {
    fn collect_garbage(&self) -> Result<()> {
        self.datastore.collect_garbage()
    }
}

impl<DS: PersistentDataStore> Persistent for BasicBatchDataStore<DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.datastore.disk_usage()
    }
}

impl<DS: ScrubbedDataStore> Scrub for BasicBatchDataStore<DS> {
    fn scrub(&self) -> Result<()> {
        self.datastore.scrub()
    }
}

impl<DS: DataStore> ToTxn for BasicBatchDataStore<DS> {
    type Txn = BasicTxnDataStore<DS>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(BasicTxnDataStore {
            datastore: self.datastore.clone(),
            ops: self.ops.clone(),
        })
    }
}

// ============================================================================

/// BasicTxnDataStore implements the transaction interface for data stores
/// who do not have any sort of underlying transaction support.
#[derive(Clone)]
pub struct BasicTxnDataStore<DS: DataStore> {
    ops: HashMap<Key, Op>,
    datastore: DS,
}

impl<DS: DataStore> BasicTxnDataStore<DS> {
    /// Create a new basic transaction datastore.
    pub fn new(datastore: DS) -> Self {
        Self {
            ops: HashMap::new(),
            datastore,
        }
    }
}

impl<DS: DataStore> DataStoreRead for BasicTxnDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Option<Vec<u8>>>
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
}

impl<DS: DataStore> DataStoreWrite for BasicTxnDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.ops.insert(key.into(), Op::Put(value.into()));
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.ops.insert(key.borrow().to_owned(), Op::Delete);
        Ok(())
    }
}

impl<DS: DataStore> DataStoreBatch for BasicTxnDataStore<DS> {
    fn commit(&mut self) -> Result<()> {
        for (key, op) in &self.ops {
            match op {
                Op::Put(value) => self.datastore.put(key, value.to_owned())?,
                Op::Delete => self.datastore.delete(&key)?,
            }
        }
        self.ops.clear();
        Ok(())
    }
}

impl<DS: DataStore> DataStoreTxn for BasicTxnDataStore<DS> {
    fn discard(&mut self) -> Result<()> {
        self.ops.clear();
        Ok(())
    }
}

impl<DS: CheckedDataStore> Check for BasicTxnDataStore<DS> {
    fn check(&self) -> Result<()> {
        self.datastore.check()
    }
}

impl<DS: GcDataStore> Gc for BasicTxnDataStore<DS> {
    fn collect_garbage(&self) -> Result<()> {
        self.datastore.collect_garbage()
    }
}

impl<DS: PersistentDataStore> Persistent for BasicTxnDataStore<DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.datastore.disk_usage()
    }
}

impl<DS: ScrubbedDataStore> Scrub for BasicTxnDataStore<DS> {
    fn scrub(&self) -> Result<()> {
        self.datastore.scrub()
    }
}

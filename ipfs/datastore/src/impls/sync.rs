// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::error::Result;
use crate::key::Key;
use crate::store::{BatchDataStore, ToBatch, ToTxn, TxnDataStore};
use crate::store::{Check, CheckedBatchDataStore, CheckedDataStore, CheckedTxnDataStore};
use crate::store::{DataStore, DataStoreBatch, DataStoreRead, DataStoreTxn, DataStoreWrite};
use crate::store::{Gc, GcBatchDataStore, GcDataStore, GcTxnDataStore};
use crate::store::{
    Persistent, PersistentBatchDataStore, PersistentDataStore, PersistentTxnDataStore,
};
use crate::store::{Scrub, ScrubbedBatchDataStore, ScrubbedDataStore, ScrubbedTxnDataStore};

/// SyncDataStore contains a datastore wrapper using rwlock.
#[derive(Clone)]
pub struct SyncDataStore<DS: DataStore> {
    datastore: Arc<RwLock<DS>>,
}

impl<DS: DataStore> SyncDataStore<DS> {
    /// Create a new datastore with a coarse lock around the entire datastore,
    /// for every single operation.
    pub fn new(datastore: DS) -> Self {
        Self {
            datastore: Arc::new(RwLock::new(datastore)),
        }
    }
}

impl<DS: DataStore> DataStore for SyncDataStore<DS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.write().sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.write().close()
    }
}

impl<DS: DataStore> DataStoreRead for SyncDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for SyncDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.write().put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.write().delete(key)
    }
}

impl<DS: CheckedDataStore> Check for SyncDataStore<DS> {
    fn check(&self) -> Result<()> {
        self.datastore.read().check()
    }
}

impl<DS: GcDataStore> Gc for SyncDataStore<DS> {
    fn collect_garbage(&self) -> Result<()> {
        self.datastore.read().collect_garbage()
    }
}

impl<DS: PersistentDataStore> Persistent for SyncDataStore<DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.datastore.read().disk_usage()
    }
}

impl<DS: ScrubbedDataStore> Scrub for SyncDataStore<DS> {
    fn scrub(&self) -> Result<()> {
        self.datastore.read().scrub()
    }
}

impl<BDS: BatchDataStore> ToBatch for SyncDataStore<BDS> {
    type Batch = SyncBatchDataStore<BDS>;

    fn batch(&self) -> Result<Self::Batch> {
        Ok(SyncBatchDataStore {
            datastore: self.datastore.clone(),
        })
    }
}

impl<TDS: TxnDataStore> ToTxn for SyncDataStore<TDS> {
    type Txn = SyncTxnDataStore<TDS>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(SyncTxnDataStore {
            datastore: self.datastore.clone(),
        })
    }
}

// ============================================================================

/// SyncBatchDataStore contains a datastore wrapper using rwlock.
#[derive(Clone)]
pub struct SyncBatchDataStore<BDS: BatchDataStore> {
    datastore: Arc<RwLock<BDS>>,
}

impl<BDS: BatchDataStore> SyncBatchDataStore<BDS> {
    /// Create a new datastore with a coarse lock around the entire datastore,
    /// for batching operations.
    pub fn new(datastore: BDS) -> Self {
        Self {
            datastore: Arc::new(RwLock::new(datastore)),
        }
    }
}

impl<BDS: BatchDataStore> DataStore for SyncBatchDataStore<BDS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.write().sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.write().close()
    }
}

impl<BDS: BatchDataStore> DataStoreRead for SyncBatchDataStore<BDS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().size(key)
    }
}

impl<BDS: BatchDataStore> DataStoreWrite for SyncBatchDataStore<BDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.write().put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.write().delete(key)
    }
}

impl<BDS: BatchDataStore> DataStoreBatch for SyncBatchDataStore<BDS> {
    fn commit(&mut self) -> Result<()> {
        self.datastore.write().commit()
    }
}

impl<BDS: CheckedBatchDataStore> Check for SyncBatchDataStore<BDS> {
    fn check(&self) -> Result<()> {
        self.datastore.read().check()
    }
}

impl<BDS: GcBatchDataStore> Gc for SyncBatchDataStore<BDS> {
    fn collect_garbage(&self) -> Result<()> {
        self.datastore.read().collect_garbage()
    }
}

impl<BDS: PersistentBatchDataStore> Persistent for SyncBatchDataStore<BDS> {
    fn disk_usage(&self) -> Result<u64> {
        self.datastore.read().disk_usage()
    }
}

impl<BDS: ScrubbedBatchDataStore> Scrub for SyncBatchDataStore<BDS> {
    fn scrub(&self) -> Result<()> {
        self.datastore.read().scrub()
    }
}

impl<TDS: TxnDataStore> ToTxn for SyncBatchDataStore<TDS> {
    type Txn = SyncTxnDataStore<TDS>;

    fn txn(&self, _read_only: bool) -> Result<Self::Txn> {
        Ok(SyncTxnDataStore {
            datastore: self.datastore.clone(),
        })
    }
}

// ============================================================================

/// SyncTxnDataStore contains a datastore wrapper using rwlock.
#[derive(Clone)]
pub struct SyncTxnDataStore<TDS: TxnDataStore> {
    datastore: Arc<RwLock<TDS>>,
}

impl<TDS: TxnDataStore> SyncTxnDataStore<TDS> {
    /// Create a new datastore with a coarse lock around the entire datastore,
    /// for batching operations.
    pub fn new(datastore: TDS) -> Self {
        Self {
            datastore: Arc::new(RwLock::new(datastore)),
        }
    }
}

impl<TDS: TxnDataStore> DataStore for SyncTxnDataStore<TDS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.write().sync(prefix)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.write().close()
    }
}

impl<TDS: TxnDataStore> DataStoreRead for SyncTxnDataStore<TDS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.datastore.read().size(key)
    }
}

impl<TDS: TxnDataStore> DataStoreWrite for SyncTxnDataStore<TDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.datastore.write().put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.datastore.write().delete(key)
    }
}

impl<TDS: TxnDataStore> DataStoreBatch for SyncTxnDataStore<TDS> {
    fn commit(&mut self) -> Result<()> {
        self.datastore.write().commit()
    }
}

impl<TDS: TxnDataStore> DataStoreTxn for SyncTxnDataStore<TDS> {
    fn discard(&mut self) -> Result<()> {
        self.datastore.write().discard()
    }
}

impl<TDS: CheckedTxnDataStore> Check for SyncTxnDataStore<TDS> {
    fn check(&self) -> Result<()> {
        self.datastore.read().check()
    }
}

impl<TDS: GcTxnDataStore> Gc for SyncTxnDataStore<TDS> {
    fn collect_garbage(&self) -> Result<()> {
        self.datastore.read().collect_garbage()
    }
}

impl<TDS: PersistentTxnDataStore> Persistent for SyncTxnDataStore<TDS> {
    fn disk_usage(&self) -> Result<u64> {
        self.datastore.read().disk_usage()
    }
}

impl<TDS: ScrubbedTxnDataStore> Scrub for SyncTxnDataStore<TDS> {
    fn scrub(&self) -> Result<()> {
        self.datastore.read().scrub()
    }
}

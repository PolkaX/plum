// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! A datastore with RocksDB as backend.

#![deny(missing_docs)]

mod rocks;

pub use self::rocks::{
    DBKey, DBOp, DBTransaction, DBValue, Database, DatabaseConfig, IoStats, IoStatsKind,
    RocksDBStatsValue, DB_DEFAULT_MEMORY_BUDGET_MB, DEFAULT_COLUMN_NAME,
};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use ipfs_datastore::{
    DataStore, DataStoreBatch, DataStoreRead, DataStoreTxn, DataStoreWrite, Key, ToBatch, ToTxn,
};

/// RocksDBDataStore is a datastore with RocksDB as backend.
#[derive(Clone)]
pub struct RocksDBDataStore {
    db: Arc<Database>,
}

impl RocksDBDataStore {
    /// Create a new rocksdb data store.
    pub fn new(config: &DatabaseConfig, path: &str) -> io::Result<Self> {
        let db = Database::open(config, path)?;
        Ok(Self { db: Arc::new(db) })
    }

    /// Get the rocksdb handle.
    pub fn db(&self) -> Arc<Database> {
        self.db.clone()
    }

    /// Add a new column family into rocksdb.
    pub fn add_column(&self, col: String) -> io::Result<()> {
        Ok(self.db.add_column(col)?)
    }

    /// Remove a column family from rocksdb.
    pub fn remove_column(&self, col: &str) -> io::Result<()> {
        Ok(self.db.remove_column(col)?)
    }

    /// The number of column families in the rocksdb.
    pub fn num_columns(&self) -> u32 {
        self.db.num_columns()
    }

    /// The number of keys in a column (estimated).
    pub fn num_keys(&self, col: &str) -> io::Result<u64> {
        Ok(self.db.num_keys(col)?)
    }

    /// Get RocksDB statistics.
    pub fn get_statistics(&self) -> HashMap<String, RocksDBStatsValue> {
        self.db.get_statistics()
    }

    // FIXME: some problems
    /// Query statistics.
    pub fn io_stats(&self, kind: IoStatsKind) -> IoStats {
        self.db.io_stats(kind)
    }
}

impl DataStore for RocksDBDataStore {
    fn sync<K>(&mut self, _prefix: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        Ok(())
    }

    fn close(&mut self) -> io::Result<()> {
        self.db.close();
        Ok(())
    }
}

impl DataStoreRead for RocksDBDataStore {
    fn get<K>(&self, key: &K) -> io::Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        Ok(self.db.get(&col, key.as_bytes())?)
    }

    fn has<K>(&self, key: &K) -> io::Result<bool>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        Ok(self.db.get(&col, key.as_bytes())?.is_some())
    }
}

impl DataStoreWrite for RocksDBDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = key.into();
        let value = value.into();
        let col = key_column(&key);

        let mut txn = self.db.transaction();
        txn.put(&col, key.as_bytes(), value);
        self.db.write(&txn)?;
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        let mut txn = self.db.transaction();
        txn.delete(&col, key.as_bytes());
        self.db.write(&txn)?;
        Ok(())
    }
}

impl ToBatch for RocksDBDataStore {
    type Batch = RocksDBBatchDataStore;

    fn batch(&self) -> io::Result<Self::Batch> {
        let db = self.db.clone();
        let txn = db.transaction();
        Ok(RocksDBBatchDataStore { db, txn })
    }
}

impl ToTxn for RocksDBDataStore {
    type Txn = RocksDBTxnDataStore;

    fn txn(&self, _read_only: bool) -> io::Result<Self::Txn> {
        let db = self.db.clone();
        let txn = db.transaction();
        Ok(RocksDBTxnDataStore { db, txn })
    }
}

// ============================================================================

/// RocksDBBatchDataStore is a batch datastore with RocksDB as backend.
#[derive(Clone)]
pub struct RocksDBBatchDataStore {
    db: Arc<Database>,
    txn: DBTransaction,
}

impl RocksDBBatchDataStore {
    /// Create a new rocksdb batch data store.
    pub fn new(config: &DatabaseConfig, path: &str) -> io::Result<Self> {
        let db = Database::open(config, path)?;
        let txn = db.transaction();
        Ok(Self {
            db: Arc::new(db),
            txn,
        })
    }

    /// Get the rocksdb handle.
    pub fn db(&self) -> Arc<Database> {
        self.db.clone()
    }
}

impl DataStoreRead for RocksDBBatchDataStore {
    fn get<K>(&self, key: &K) -> io::Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        Ok(self.db.get(&col, key.as_bytes())?)
    }

    fn has<K>(&self, key: &K) -> io::Result<bool>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        Ok(self.db.get(&col, key.as_bytes())?.is_some())
    }
}

impl DataStoreWrite for RocksDBBatchDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = key.into();
        let value = value.into();
        let col = key_column(&key);

        self.txn.put(&col, key.as_bytes(), value);
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        self.txn.delete(&col, key.borrow().as_bytes());
        Ok(())
    }
}

impl DataStoreBatch for RocksDBBatchDataStore {
    fn commit(&mut self) -> io::Result<()> {
        self.db.write(&self.txn)?;
        self.txn.clear();
        Ok(())
    }
}

impl ToTxn for RocksDBBatchDataStore {
    type Txn = RocksDBTxnDataStore;

    fn txn(&self, _read_only: bool) -> io::Result<Self::Txn> {
        Ok(RocksDBTxnDataStore {
            db: self.db.clone(),
            txn: self.txn.clone(),
        })
    }
}

// ============================================================================

/// RocksDBBatchDataStore is a txn datastore with RocksDB as backend.
#[derive(Clone)]
pub struct RocksDBTxnDataStore {
    db: Arc<Database>,
    txn: DBTransaction,
}

impl RocksDBTxnDataStore {
    /// Create a new rocksdb batch data store.
    pub fn new(config: &DatabaseConfig, path: &str) -> io::Result<Self> {
        let db = Database::open(config, path)?;
        let txn = db.transaction();
        Ok(Self {
            db: Arc::new(db),
            txn,
        })
    }

    /// Get the rocksdb handle.
    pub fn db(&self) -> Arc<Database> {
        self.db.clone()
    }
}

impl DataStoreRead for RocksDBTxnDataStore {
    fn get<K>(&self, key: &K) -> io::Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        Ok(self.db.get(&col, key.as_bytes())?)
    }

    fn has<K>(&self, key: &K) -> io::Result<bool>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        Ok(self.db.get(&col, key.as_bytes())?.is_some())
    }
}

impl DataStoreWrite for RocksDBTxnDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = key.into();
        let value = value.into();

        let col = key_column(&key);
        self.txn.put(&col, key.as_bytes(), value);
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> io::Result<()>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        let col = key_column(key);

        self.txn.delete(&col, key.as_bytes());
        Ok(())
    }
}

impl DataStoreBatch for RocksDBTxnDataStore {
    fn commit(&mut self) -> io::Result<()> {
        self.db.write(&self.txn)?;
        self.txn.clear();
        Ok(())
    }
}

impl DataStoreTxn for RocksDBTxnDataStore {
    fn discard(&mut self) -> io::Result<()> {
        self.txn.ops.clear();
        Ok(())
    }
}

// TODO: specified col name according to the key.
// Get column family name according to the key.
fn key_column(_key: &Key) -> String {
    DEFAULT_COLUMN_NAME.to_string()
}

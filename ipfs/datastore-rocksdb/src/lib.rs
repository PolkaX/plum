// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod rocks;

pub use self::rocks::{
    DBKey, DBOp, DBTransaction, DBValue, DatabaseConfig, IoStats, IoStatsKind, RocksDBStatsValue,
    DB_DEFAULT_MEMORY_BUDGET_MB,
};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use ipfs_datastore::{DataStore, DataStoreError, DataStoreRead, DataStoreWrite, Key};

pub(crate) type Result<T> = std::result::Result<T, DataStoreError>;

/// RocksDBDataStore is a datastore with RocksDB as backend.
#[derive(Clone)]
pub struct RocksDBDataStore {
    db: Arc<rocks::Database>,
}

impl RocksDBDataStore {
    /// Create a new rocksdb data store.
    pub fn new(config: &DatabaseConfig, path: &str) -> Result<Self> {
        let db = rocks::Database::open(config, path)?;
        Ok(Self { db: Arc::new(db) })
    }

    /// Add a new column family into rocksdb.
    pub fn add_column(&self, col: String) -> Result<()> {
        Ok(self.db.add_column(col)?)
    }

    /// Remove a column family from rocksdb.
    pub fn remove_column(&self, col: &str) -> Result<()> {
        Ok(self.db.remove_column(col)?)
    }

    /// The number of column families in the rocksdb.
    pub fn num_columns(&self) -> u32 {
        self.db.num_columns()
    }

    /// The number of keys in a column (estimated).
    pub fn num_keys(&self, col: &str) -> Result<u64> {
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
    fn sync<K>(&mut self, _prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        self.db.close();
        Ok(())
    }
}

impl DataStoreRead for RocksDBDataStore {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        // TODO: specified col name.
        self.db
            .get("", key.borrow().as_bytes())?
            .ok_or_else(|| DataStoreError::NotFound(key.borrow().to_string()))
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        // TODO: specified col name.
        Ok(self.db.get("", key.borrow().as_bytes())?.is_some())
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        // TODO: specified col name.
        self.db
            .get("", key.borrow().as_bytes())?
            .ok_or_else(|| DataStoreError::NotFound(key.borrow().to_string()))
            .map(|value| value.len())
    }
}

impl DataStoreWrite for RocksDBDataStore {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let mut txn = self.db.transaction();
        txn.put("", key.into().as_bytes(), value.into());
        self.db.write(txn)?;
        Ok(())
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        let mut txn = self.db.transaction();
        txn.delete("", key.borrow().as_bytes());
        self.db.write(txn)?;
        Ok(())
    }
}

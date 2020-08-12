// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::io;

use crate::store::{BatchDataStore, DataStore, TxnDataStore};

/// An interface that report disk usage.
pub trait Persistent {
    /// Report disk usage, return the space used by a datastore, in bytes.
    fn disk_usage(&self) -> io::Result<u64>;
}

/// PersistentDataStore is an interface that should be implemented by data stores
/// which can report disk usage.
pub trait PersistentDataStore: Persistent + DataStore {}
impl<T: Persistent + DataStore> PersistentDataStore for T {}

/// PersistentBatchDataStore is an interface that should be implemented by batch data stores
/// which can report disk usage.
pub trait PersistentBatchDataStore: Persistent + BatchDataStore {}
impl<T: Persistent + BatchDataStore> PersistentBatchDataStore for T {}

/// PersistentTxnDataStore is an interface that should be implemented by txn data stores
/// which can report disk usage.
pub trait PersistentTxnDataStore: Persistent + TxnDataStore {}
impl<T: Persistent + TxnDataStore> PersistentTxnDataStore for T {}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Result;
use crate::store::{BatchDataStore, DataStore, TxnDataStore};

/// An interface that free disk space.
pub trait Gc {
    /// Free disk space.
    fn collect_garbage(&self) -> Result<()>;
}

/// GcDataStore is an interface that should be implemented by data stores
/// which don't free disk space by just removing data from them.
pub trait GcDataStore: Gc + DataStore {}
impl<T: Gc + DataStore> GcDataStore for T {}

/// GcBatchDataStore is an interface that should be implemented by batch data stores
/// which don't free disk space by just removing data from them.
pub trait GcBatchDataStore: Gc + BatchDataStore {}
impl<T: Gc + BatchDataStore> GcBatchDataStore for T {}

/// GcTxnDataStore is an interface that should be implemented by txn data stores
/// which don't free disk space by just removing data from them.
pub trait GcTxnDataStore: Gc + TxnDataStore {}
impl<T: Gc + TxnDataStore> GcTxnDataStore for T {}

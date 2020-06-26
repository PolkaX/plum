// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Result;
use crate::store::{BatchDataStore, DataStore, TxnDataStore};

/// An interface check on-disk data integrity.
pub trait Check {
    /// Check on-disk data integrity.
    fn check(&self) -> Result<()>;
}

/// CheckedDataStore is an interface that should be implemented by data stores
/// which may need checking on-disk data integrity.
pub trait CheckedDataStore: Check + DataStore {}
impl<T: Check + DataStore> CheckedDataStore for T {}

/// CheckedBatchDataStore is an interface that should be implemented by batch data stores
/// which may need checking on-disk data integrity.
pub trait CheckedBatchDataStore: Check + BatchDataStore {}
impl<T: Check + BatchDataStore> CheckedBatchDataStore for T {}

/// CheckedTxnDataStore is an interface that should be implemented by txn data stores
/// which may need checking on-disk data integrity.
pub trait CheckedTxnDataStore: Check + TxnDataStore {}
impl<T: Check + TxnDataStore> CheckedTxnDataStore for T {}

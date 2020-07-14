// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::io::Result;

use crate::store::{BatchDataStore, DataStore, TxnDataStore};

/// An interface that check data integrity and/or error correction.
pub trait Scrub {
    /// Check data integrity and/or error correction.
    fn scrub(&self) -> Result<()>;
}

/// ScrubbedDataStore is an interface that should be implemented by data stores
/// which want to provide a mechanism to check data integrity and/or error correction.
pub trait ScrubbedDataStore: Scrub + DataStore {}
impl<T: Scrub + DataStore> ScrubbedDataStore for T {}

/// ScrubbedBatchDataStore is an interface that should be implemented by batch data stores
/// which want to provide a mechanism to check data integrity and/or error correction.
pub trait ScrubbedBatchDataStore: Scrub + BatchDataStore {}
impl<T: Scrub + BatchDataStore> ScrubbedBatchDataStore for T {}

/// ScrubbedTxnDataStore is an interface that should be implemented by txn data stores
/// which want to provide a mechanism to check data integrity and/or error correction.
pub trait ScrubbedTxnDataStore: Scrub + TxnDataStore {}
impl<T: Scrub + TxnDataStore> ScrubbedTxnDataStore for T {}

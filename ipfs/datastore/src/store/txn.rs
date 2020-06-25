// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::store::{Batch, DataStore, DataStoreRead, DataStoreWrite};

/// Txn extends the DataStore type.
///
/// Txn allow users to batch queries and mutations to the DataStore into atomic groups, or transactions.
/// Actions performed on a transaction will not take hold until a successful call to `commit` has been made.
/// Likewise, transactions can be aborted by calling `discard` before a successful Commit has been made.
pub trait Txn: DataStoreRead + DataStoreWrite + Batch {
    /// Discard throws away changes recorded in a transaction without committing them to the
    /// underlying DataStore.
    /// Any calls made to Discard after Commit has been successfully called will have no effect on
    /// the transaction and state of the DataStore, making it safe to defer.
    fn discard(&mut self);
}

/// TxnDataStore is an interface that should be implemented by data stores that support transactions.
pub trait TxnDataStore: Txn + DataStore {}

impl<T: Txn + DataStore> TxnDataStore for T {}

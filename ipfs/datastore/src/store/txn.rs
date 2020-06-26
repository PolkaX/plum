// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Result;
use crate::store::DataStoreTxn;

/// Txn is an interface that should be implemented by data stores that support transactions.
pub trait Txn {
    /// The txn type returned by the `new_txn` method.
    type Txn: DataStoreTxn;

    /// Create a new transaction.
    fn new_txn(&self, read_only: bool) -> Result<Self::Txn>;
}

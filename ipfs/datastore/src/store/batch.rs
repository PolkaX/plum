// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::store::{DataStore, DataStoreWrite};

/// BatchDataStore support deferred, grouped updates to the database.
/// `Batch`es do NOT have transactional semantics: updates to the underlying
/// datastore are not guaranteed to occur in the same iota of time.
/// Similarly, batched updates will not be flushed to the underlying datastore
/// until `Commit` has been called.
/// `Txn`s from a `TxnDataStore` have all the capabilities of a `Batch`,
/// but the reverse is NOT true.
pub trait BatchDataStore: DataStore {
    ///
    type Batch: Batch;

    ///
    fn batch(&self) -> Result<Self::Batch, ()>;
}

///
pub trait Batch: DataStoreWrite {
    ///
    fn commit(&self) -> Result<(), ()>;
}

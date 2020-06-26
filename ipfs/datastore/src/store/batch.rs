// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Result;
use crate::store::DataStoreBatch;

/// Batch is an interface that should be implemented by data stores that
/// support deferred, grouped updates to the database.
pub trait Batch {
    /// The batch type returned by the `batch` method.
    type Batch: DataStoreBatch;

    /// Create a new batching data store.
    fn batch(&self) -> Result<Self::Batch>;
}

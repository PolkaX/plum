// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Result;
use crate::store::{DataStore, DataStoreWrite};

/// An interface that support batched update operations.
pub trait Batch: DataStoreWrite {
    /// Commit all update operations.
    fn commit(&mut self) -> Result<()>;
}

/// BatchDataStore is an interface that should be implemented by data stores
/// which need to support batched update operations.
pub trait BatchDataStore: Batch + DataStore {}

impl<T: Batch + DataStore> BatchDataStore for T {}

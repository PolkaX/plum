// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::store::DataStore;

/// PersistentDataStore is an interface that should be implemented by data stores
/// which can report disk usage.
pub trait PersistentDataStore: DataStore {
    /// Report disk usage, return the space used by a datastore, in bytes.
    fn disk_usage(&self) -> Result<u64, ()>;
}

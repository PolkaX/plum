// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::store::DataStore;

/// GcDataStore is an interface that should be implemented by data stores
/// which don't free disk space by just removing data from them.
pub trait GcDataStore: DataStore {
    /// Free disk space.
    fn collect_garbage(&self) -> Result<(), ()>;
}

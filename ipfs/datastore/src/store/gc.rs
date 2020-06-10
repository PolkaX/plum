// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Result;
use crate::store::DataStore;

///
pub trait Gc {
    /// Free disk space.
    fn collect_garbage(&self) -> Result<()>;
}

/// GcDataStore is an interface that should be implemented by data stores
/// which don't free disk space by just removing data from them.
pub trait GcDataStore: Gc + DataStore {}

impl<T: Gc + DataStore> GcDataStore for T {}

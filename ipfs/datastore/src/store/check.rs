// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::store::DataStore;

///  CheckedDataStore is an interface that should be implemented by data stores
/// which may need checking on-disk data integrity.
pub trait CheckedDataStore: DataStore {
    /// Check on-disk data integrity.
    fn check(&self) -> Result<(), ()>;
}

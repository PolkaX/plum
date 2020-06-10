// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::store::DataStore;

/// ScrubbedDataStore is an interface that should be implemented by data stores
/// which want to provide a mechanism to check data integrity and/or error correction.
pub trait ScrubbedDataStore: DataStore {
    /// Check data integrity and/or error correction.
    fn scrub(&self) -> Result<(), ()>;
}

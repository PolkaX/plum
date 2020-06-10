// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::time::Instant;

use crate::error::Result;
use crate::store::DataStore;

/// Ttl encapsulates the methods that deal with entries with time-to-live.
pub trait Ttl {
    ///
    fn put_with_ttl(&self) -> Result<()>;
    ///
    fn set_ttl(&self) -> Result<()>;
    ///
    fn get_expiration(&self) -> Result<Instant>;
}

/// TtlDataStore is an interface that should be implemented by data stores
/// that support expiring entries.
pub trait TtlDataStore: Ttl + DataStore {}

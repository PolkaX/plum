// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::time::{Duration, Instant};

use crate::error::Result;
use crate::key::Key;
use crate::store::DataStore;

/// Ttl encapsulates the methods that deal with entries with time-to-live.
pub trait Ttl {
    /// Store the object `value` named by `key` with time-to-live.
    fn put_with_ttl<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>;

    /// Set the duration of time-to-live.
    fn set_ttl(&mut self, ttl: Duration) -> Result<()>;

    /// Get the next expiration of time-to-live of the `key`.
    fn get_expiration<K>(&self, key: &K) -> Result<Instant>
    where
        K: Borrow<Key>;
}

/// TtlDataStore is an interface that should be implemented by data stores
/// that support expiring entries.
pub trait TtlDataStore: Ttl + DataStore {}

impl<T: Ttl + DataStore> TtlDataStore for T {}

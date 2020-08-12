// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::io;
use std::time::{Duration, Instant};

use crate::key::Key;
use crate::store::{BatchDataStore, DataStore, TxnDataStore};

/// Ttl encapsulates the methods that deal with entries with time-to-live.
pub trait Ttl {
    /// Store the object `value` named by `key` with time-to-live.
    fn put_with_ttl<K, V>(&mut self, key: K, value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>;

    /// Set the duration of time-to-live.
    fn set_ttl(&mut self, ttl: Duration) -> io::Result<()>;

    /// Get the next expiration of time-to-live of the `key`.
    fn get_expiration<K>(&self, key: &K) -> io::Result<Instant>
    where
        K: Borrow<Key>;
}

/// TtlDataStore is an interface that should be implemented by data stores
/// that support expiring entries.
pub trait TtlDataStore: Ttl + DataStore {}
impl<T: Ttl + DataStore> TtlDataStore for T {}

/// TtlBatchDataStore is an interface that should be implemented by batch data stores
/// that support expiring entries.
pub trait TtlBatchDataStore: Ttl + BatchDataStore {}
impl<T: Ttl + BatchDataStore> TtlBatchDataStore for T {}

/// TtlTxnDataStore is an interface that should be implemented by txn data stores
/// that support expiring entries.
pub trait TtlTxnDataStore: Ttl + TxnDataStore {}
impl<T: Ttl + TxnDataStore> TtlTxnDataStore for T {}

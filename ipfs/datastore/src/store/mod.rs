// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use crate::error::Result;
use crate::key::Key;

/// DataStore represents storage for any key-value pair.
///
/// DataStores are general enough to be backed by all kinds of different storage:
/// in-memory caches, databases, a remote datastore, flat files on disk, etc.
///
/// The general idea is to wrap a more complicated storage facility in a simple,
/// uniform interface, keeping the freedom of using the right tools for the job.
/// In particular, a DataStore can aggregate other data stores in interesting ways,
/// like sharded (to distribute load) or tiered access (caches before databases).
///
/// While DataStores should be written general enough to accept all sorts of
/// values, some implementations will undoubtedly have to be specific (e.g. SQL
/// databases where fields should be decomposed into columns), particularly to
/// support queries efficiently. Moreover, certain data stores may enforce certain
/// types of values (e.g. requiring an io.Reader, a specific struct, etc) or
/// serialization formats (JSON, Protobuf, etc).
///
/// IMPORTANT: No DataStore should ever Panic! This is a cross-module interface,
/// and thus it should behave predictably and handle exceptional conditions with
/// proper error reporting. Thus, all DataStore calls may return errors, which
/// should be checked by callers.
pub trait DataStore: DataStoreWrite + DataStoreRead {
    ///
    fn sync<K>(&self, prefix: K) -> Result<()>
    where
        K: Into<Key>;

    ///
    fn close(&self) -> Result<()>;
}

/// DataStoreWrite is the write-side of the DataStore trait.
pub trait DataStoreWrite {
    /// Store the object `value` named by `key`.
    ///
    /// The generalized DataStore interface does not impose a value type,
    /// allowing various datastore middleware implementations (which do not
    /// handle the values directly) to be composed together.
    ///
    /// Ultimately, the lowest-level datastore will need to do some value checking
    /// or risk getting incorrect values. It may also be useful to expose a more
    /// type-safe interface to your application, and do the checking up-front.
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>;

    /// Remove the value for given `key`.
    /// If the key is not in the datastore, this method returns no error.
    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>;
}

/// DataStoreRead is the read-side of the DataStore trait.
pub trait DataStoreRead {
    /// Retrieve the object `value` named by `key`.
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>;

    /// Return whether the `key` is mapped to a `value`.
    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>;

    /// Return the size of the `value` named by `key`.
    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>;

    // Query searches the datastore and returns a query result. This function
    // may return before the query actually runs.
    // TODO: query
}

// ============================================================================
// ********************** Extended DataStore interfaces ***********************
// ============================================================================

mod batch;
mod check;
mod gc;
mod persistent;
mod scrub;
mod ttl;
mod txn;

pub use self::batch::{Batch, BatchDataStore};
pub use self::check::{Check, CheckedDataStore};
pub use self::gc::{Gc, GcDataStore};
pub use self::persistent::{Persistent, PersistentDataStore};
pub use self::scrub::{Scrub, ScrubbedDataStore};
pub use self::ttl::{Ttl, TtlDataStore};
pub use self::txn::{Txn, TxnDataStore};

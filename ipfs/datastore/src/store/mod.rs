// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::key::Key;

/// DataStore represents storage for any key-value pair.
///
/// DataStores are general enough to be backed by all kinds of different storage:
/// in-memory caches, databases, a remote datastore, flat files on disk, etc.
///
/// The general idea is to wrap a more complicated storage facility in a simple,
/// uniform interface, keeping the freedom of using the right tools for the job.
/// In particular, a Datastore can aggregate other data stores in interesting ways,
/// like sharded (to distribute load) or tiered access (caches before databases).
///
/// While DataStores should be written general enough to accept all sorts of
/// values, some implementations will undoubtedly have to be specific (e.g. SQL
/// databases where fields should be decomposed into columns), particularly to
/// support queries efficiently. Moreover, certain data stores may enforce certain
/// types of values (e.g. requiring an io.Reader, a specific struct, etc) or
/// serialization formats (JSON, Protobuf, etc).
///
/// IMPORTANT: No Datastore should ever Panic! This is a cross-module interface,
/// and thus it should behave predictably and handle exceptional conditions with
/// proper error reporting. Thus, all Datastore calls may return errors, which
/// should be checked by callers.
pub trait DataStore: DataStoreWrite + DataStoreRead {
    ///
    fn sync<K: Into<Key>>(&self, prefix: K) -> Result<(), ()>;

    ///
    fn close(&self) -> Result<(), ()>;
}

/// DataStoreWrite is the write-side of the DataStore trait.
pub trait DataStoreWrite {
    ///
    fn put<K, V>(&self, key: K, value: V) -> Result<(), ()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>;

    ///
    fn delete<K: ?Sized>(&self, key: &K) -> Result<(), ()>;
}

/// DataStoreRead is the read-side of the DataStore trait.
pub trait DataStoreRead {
    /// Retrieve the object `value` named by `key`.
    fn get<K: ?Sized>(&self, key: &K) -> Option<Vec<u8>>;

    /// Return whether the `key` is mapped to a `value`.
    fn has<K: ?Sized>(&self, key: &K) -> bool;

    /// Return the size of the `value` named by `key`.
    fn size<K: ?Sized>(&self, key: &K) -> Option<usize>;

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
pub use self::check::CheckedDataStore;
pub use self::gc::GcDataStore;
pub use self::persistent::PersistentDataStore;
pub use self::scrub::ScrubbedDataStore;
pub use self::ttl::{Ttl, TtlDataStore};
pub use self::txn::{Txn, TxnDataStore};

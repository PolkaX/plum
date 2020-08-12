// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;
use std::io;

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
pub trait DataStore: DataStoreRead + DataStoreWrite + Clone {
    /// Guarantees that any `put` or `delete` calls under prefix that returned before `sync(prefix)`
    /// was called will be observed after `sync(prefix)` returns, even if the program crashes.
    /// If `put/delete` operations already satisfy these requirements then Sync may be a no-op.
    ///
    ///  If the prefix fails to `sync` this method returns an error.
    fn sync<K>(&mut self, prefix: &K) -> io::Result<()>
    where
        K: Borrow<Key>;

    /// Close I/O.
    fn close(&mut self) -> io::Result<()>;
}

/// DataStoreRead is the read-side of the DataStore trait.
pub trait DataStoreRead {
    /// Retrieve the object `value` named by `key`.
    fn get<K>(&self, key: &K) -> io::Result<Option<Vec<u8>>>
    where
        K: Borrow<Key>;

    /// Return whether the `key` is mapped to a `value`.
    fn has<K>(&self, key: &K) -> io::Result<bool>
    where
        K: Borrow<Key>;

    // Query searches the datastore and returns a query result. This function
    // may return before the query actually runs.
    // TODO: query
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
    fn put<K, V>(&mut self, key: K, value: V) -> io::Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>;

    /// Remove the value for given `key`.
    /// If the key is not in the datastore, this method returns no error.
    fn delete<K>(&mut self, key: &K) -> io::Result<()>
    where
        K: Borrow<Key>;
}

/// DataStoreBatch is a interface that needs to be implemented by `BatchDataStore`
/// to support batch write.
pub trait DataStoreBatch: DataStoreWrite {
    /// Commit all update operations.
    fn commit(&mut self) -> io::Result<()>;
}

/// BatchDataStore is an interface that should be implemented by data stores that
/// support deferred, grouped updates to the database.
pub trait BatchDataStore: DataStoreBatch + DataStore {}
impl<T: DataStoreBatch + DataStore> BatchDataStore for T {}

/// ToBatch is an interface that should be implemented by data stores that
/// support deferred, grouped updates to the database.
pub trait ToBatch {
    /// The batch type returned by the `batch` method.
    type Batch: DataStoreBatch;

    /// Create a new batching data store.
    fn batch(&self) -> io::Result<Self::Batch>;
}

/// ToBatchDataStore is an interface that describe a database have batch feature, it
/// could generate a database which is implemented by DataStoreBatch
pub trait ToBatchDataStore: ToBatch + DataStore {}
impl<T: ToBatch + DataStore> ToBatchDataStore for T {}

/// DataStoreTxn is a interface that needs to be implemented by `TxnDataStore`
/// to support transactions.
pub trait DataStoreTxn: DataStoreRead + DataStoreBatch {
    /// Discard throws away changes recorded in a transaction without committing
    /// them to the underlying Datastore. Any calls made to Discard after Commit
    /// has been successfully called will have no effect on the transaction and
    /// state of the Datastore, making it safe to defer.
    fn discard(&mut self) -> io::Result<()>;
}

/// TxnDataStore is an interface that should be implemented by data stores that support transactions.
pub trait TxnDataStore: DataStoreTxn + DataStore {}
impl<T: DataStoreTxn + DataStore> TxnDataStore for T {}

/// ToTxn is an interface that should be implemented by data stores that support transactions.
pub trait ToTxn {
    /// The txn type returned by the `txn` method.
    type Txn: DataStoreTxn;

    /// Create a new txn data store.
    fn txn(&self, read_only: bool) -> io::Result<Self::Txn>;
}

/// ToTxnDataStore is an interface that describe a database have totxn feature, it
/// could generate a database which is implemented by DataStoreTxn
pub trait ToTxnDataStore: ToTxn + DataStore {}
impl<T: ToTxn + DataStore> ToTxnDataStore for T {}

// ============================================================================
// ********************** Extended DataStore interfaces ***********************
// ============================================================================

mod check;
mod gc;
mod persistent;
mod scrub;
mod ttl;

pub use self::check::{Check, CheckedBatchDataStore, CheckedDataStore, CheckedTxnDataStore};
pub use self::gc::{Gc, GcBatchDataStore, GcDataStore, GcTxnDataStore};
pub use self::persistent::{
    Persistent, PersistentBatchDataStore, PersistentDataStore, PersistentTxnDataStore,
};
pub use self::scrub::{Scrub, ScrubbedBatchDataStore, ScrubbedDataStore, ScrubbedTxnDataStore};
pub use self::ttl::{Ttl, TtlBatchDataStore, TtlDataStore, TtlTxnDataStore};

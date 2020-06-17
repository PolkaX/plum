// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of IPFS DataStore.

#![deny(missing_docs)]

mod error;
mod impls;
mod key;
// TODO: mount and query
// mod mount;
// mod query;
mod store;

pub use self::error::DataStoreError;
pub use self::impls::BasicBatchDataStore;
pub use self::impls::{Delay, DelayDataStore};
pub use self::impls::{DummyDataStore, LogBatchDataStore, LogDataStore, MapDataStore};
pub use self::impls::{FailBatchDataStore, FailDataStore, FailFn};
pub use self::impls::{KeyMapFn, KeyTransformPair, PrefixTransform};
pub use self::impls::{KeyTransform, TransformBatchDataStore, TransformDataStore};
pub use self::impls::{SyncBatchDataStore, SyncDataStore};
pub use self::key::{namespace_type, namespace_value, Key};
pub use self::store::{Batch, BatchDataStore, Batching};
pub use self::store::{Check, CheckedDataStore};
pub use self::store::{DataStore, DataStoreRead, DataStoreWrite};
pub use self::store::{Gc, GcDataStore};
pub use self::store::{Persistent, PersistentDataStore};
pub use self::store::{Scrub, ScrubbedDataStore};
pub use self::store::{Ttl, TtlDataStore};
pub use self::store::{Txn, TxnDataStore};

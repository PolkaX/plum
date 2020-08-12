// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of IPFS DataStore.

#![deny(missing_docs)]

mod impls;
mod key;
// mod query;
mod store;

pub use self::key::{namespace_type, namespace_value, Key};
// pub use self::query::*;

pub use self::store::{DataStore, DataStoreBatch, DataStoreRead, DataStoreTxn, DataStoreWrite};

pub use self::store::{BatchDataStore, TxnDataStore};
pub use self::store::{ToBatch, ToBatchDataStore};
pub use self::store::{ToTxn, ToTxnDataStore};

pub use self::store::{Check, CheckedBatchDataStore, CheckedDataStore, CheckedTxnDataStore};
pub use self::store::{Gc, GcBatchDataStore, GcDataStore, GcTxnDataStore};
pub use self::store::{
    Persistent, PersistentBatchDataStore, PersistentDataStore, PersistentTxnDataStore,
};
pub use self::store::{Scrub, ScrubbedBatchDataStore, ScrubbedDataStore, ScrubbedTxnDataStore};
pub use self::store::{Ttl, TtlBatchDataStore, TtlDataStore, TtlTxnDataStore};

pub use self::impls::{BasicBatchDataStore, BasicTxnDataStore};
pub use self::impls::{Delay, DelayDataStore};
pub use self::impls::{DummyDataStore, MapDataStore};

pub use self::impls::{FailBatchDataStore, FailDataStore, FailFn, FailTxnDataStore};
pub use self::impls::{
    KeyMapFn, KeyTransform, KeyTransformPair, PrefixTransform, TransformBatchDataStore,
    TransformDataStore, TransformTxnDataStore,
};
pub use self::impls::{LogBatchDataStore, LogDataStore, LogTxnDataStore};
pub use self::impls::{SyncBatchDataStore, SyncDataStore, SyncTxnDataStore};

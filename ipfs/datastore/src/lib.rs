// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of IPFS DataStore.

#![deny(missing_docs)]

mod error;
mod impls;
mod key;
// TODO: query and mount
// mod mount;
// mod query;
mod store;

pub use self::error::DataStoreError;
pub use self::impls::{DummyDataStore, LogDataStore, MapDataStore, SyncDataStore};
pub use self::impls::{FailDataStore, FailFunc};
pub use self::impls::{KeyTransform, TransformDataStore};
pub use self::key::{namespace_type, namespace_value, Key};
pub use self::store::{Batch, BatchDataStore};
pub use self::store::{CheckedDataStore, GcDataStore, PersistentDataStore, ScrubbedDataStore};
pub use self::store::{DataStore, DataStoreRead, DataStoreWrite};
pub use self::store::{Ttl, TtlDataStore};
pub use self::store::{Txn, TxnDataStore};

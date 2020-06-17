// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod batch;
mod delay;
mod dummy;
mod fail;
mod log;
mod map;
// TODO: retry
// mod retry;
mod sync;
mod transform;

pub use self::batch::BasicBatchDataStore;
pub use self::delay::{Delay, DelayDataStore};
pub use self::dummy::DummyDataStore;
pub use self::fail::{FailBatchDataStore, FailDataStore, FailFn};
pub use self::log::{LogBatchDataStore, LogDataStore};
pub use self::map::MapDataStore;
pub use self::sync::{SyncBatchDataStore, SyncDataStore};
pub use self::transform::{KeyMapFn, KeyTransformPair, PrefixTransform};
pub use self::transform::{KeyTransform, TransformBatchDataStore, TransformDataStore};

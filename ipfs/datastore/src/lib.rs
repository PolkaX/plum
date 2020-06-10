// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod datastore;
mod key;
// mod query;
mod store;

pub use self::key::{namespace_type, namespace_value, Key};

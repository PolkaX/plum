// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! TipsetKey and Tipset with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod errors;
mod key;
mod tipset;

pub use self::errors::TipsetError;
pub use self::key::{cbor as tipset_key_cbor, json as tipset_key_json, TipsetKey};
pub use self::tipset::{cbor as tipset_cbor, json as tipset_json, Tipset};

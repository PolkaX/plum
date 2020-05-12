// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! TipsetKey and Tipset with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod errors;
mod key;
mod tipset;

pub use self::errors::TipsetError;
pub use self::key::TipsetKey;
pub use self::tipset::Tipset;

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! Hash types with serialization/deserialization.

#![deny(missing_docs)]

mod h256;

pub use self::h256::{option as h256_option, raw as h256_raw, H256};

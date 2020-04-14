// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! Hash types with serialization/deserialization.

#![deny(missing_docs)]

mod h256;

pub use self::h256::{cbor as h256_cbor, H256};

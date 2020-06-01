// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

#[macro_use]
mod macros;

mod cbor;
mod json;

use std::collections::BTreeMap;

use cid::Cid;

/// The IPLD value.
// See [IPLD Data Model](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md) for details.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IpldValue {
    /// Null value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// Integer value.
    Integer(i128),
    /// Floating point value.
    Float(f64),
    /// UTF-8 string value.
    String(String),
    /// Byte string value.
    Bytes(Vec<u8>),
    /// List value.
    List(Vec<IpldValue>),
    /// Map value.
    Map(BTreeMap<String, IpldValue>),
    /// Link value.
    Link(Cid),
}

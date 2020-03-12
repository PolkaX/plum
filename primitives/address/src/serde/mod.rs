// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// CBOR serialization/deserialization
pub mod cbor;
/// JSON serialization/deserialization
pub mod json;

use serde::{de, ser};

use crate::address::Address;

// Implement default serialization for Address.
impl ser::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

// Implement default deserialization for Address.
impl<'de> de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

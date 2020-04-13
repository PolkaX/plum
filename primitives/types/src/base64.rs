// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

// Fuck golang std library.

use serde::{de, ser, Deserialize, Serialize};

/// JSON serialization of Vec<u8> using base64.
pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    base64::encode(bytes).serialize(serializer)
}

/// JSON deserialization of Vec<u8> using base64.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    base64::decode(String::deserialize(deserializer)?)
        .map_err(|err| de::Error::custom(format!("base64 decode error: {}", err)))
}

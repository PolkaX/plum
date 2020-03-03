// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::str::FromStr;

use serde::{de, ser, Deserialize, Serialize};

use crate::address::Address;

/// JSON serialization
pub fn serialize<S>(address: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let value = address.to_string();
    value.serialize(serializer)
}

/// JSON deserialization
pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: de::Deserializer<'de>,
{
    let addr = String::deserialize(deserializer)?;
    Ok(Address::from_str(&addr).map_err(de::Error::custom)?)
}

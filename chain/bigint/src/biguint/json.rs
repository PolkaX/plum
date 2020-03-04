// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::str::FromStr;

use num_bigint::BigUint;
use serde::{de, ser, Deserialize, Serialize};

pub fn serialize<S>(uint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    uint.to_string().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: de::Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    Ok(BigUint::from_str(v.as_str()).map_err(|e| de::Error::custom(e.to_string()))?)
}
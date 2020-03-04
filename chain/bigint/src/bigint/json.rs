// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::str::FromStr;

use num_bigint::BigInt;
use serde::{de, ser, Deserialize, Serialize};

pub fn serialize<S>(int: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    int.to_string().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: de::Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    Ok(BigInt::from_str(v.as_str()).map_err(|e| de::Error::custom(e.to_string()))?)
}

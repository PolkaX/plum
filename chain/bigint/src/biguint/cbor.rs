// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};
use serde_bytes::{ByteBuf, Bytes};

use num_bigint::BigUint;

pub fn serialize<S>(uint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    use serde_bytes::Serialize as _;
    let mut v = uint.to_bytes_be();

    if v == [0] {
        v = Vec::new()
    } else {
        v.insert(0, 0);
    }
    let value = Bytes::new(&v);
    value.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: de::Deserializer<'de>,
{
    use serde_bytes::Deserialize as _;
    let v = ByteBuf::deserialize(deserializer)?;
    Ok(BigUint::from_bytes_be(&v))
}

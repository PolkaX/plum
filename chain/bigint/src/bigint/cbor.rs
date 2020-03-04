// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use num_bigint::{BigInt, Sign};
use serde::{de, ser};
use serde_bytes::{ByteBuf, Bytes, Deserialize, Serialize};

/// CBOR serialization
pub fn serialize<S>(int: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let (sign, mut v) = int.to_bytes_be();
    let v = match sign {
        Sign::Plus => {
            let mut buf = Vec::with_capacity(1 + v.len());
            buf.push(0);
            buf.extend(v.iter());
            buf
        }
        Sign::Minus => {
            let mut buf = Vec::with_capacity(1 + v.len());
            buf.push(1);
            buf.extend(v.iter());
            buf
        }
        Sign::NoSign => {
            v.clear();
            v
        }
    };
    let value = Bytes::new(&v);
    value.serialize(serializer)
}

/// CBOR deserialization
pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: de::Deserializer<'de>,
{
    let v = ByteBuf::deserialize(deserializer)?;
    let v = v.into_vec();
    if v.is_empty() {
        return Ok(BigInt::default());
    }
    let sign = match &v[0] {
        0 => Sign::Plus,
        1 => Sign::Minus,
        _ => {
            return Err(de::Error::custom(format!(
                "big int prefix should be either 0 or 1, got {}",
                v[0]
            )))
        }
    };
    Ok(BigInt::from_bytes_be(sign, &v[1..]))
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

/// CBOR serialization/deserialization
pub mod cbor {
    use num_bigint::BigUint;
    use serde::{de, ser};
    use serde_bytes::{ByteBuf, Bytes};

    /// CBOR serialization
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

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use serde_bytes::Deserialize as _;
        let v = ByteBuf::deserialize(deserializer)?;
        Ok(BigUint::from_bytes_be(&v))
    }
}

/// JSON serialization/deserialization
pub mod json {
    use std::str::FromStr;

    use num_bigint::BigUint;
    use serde::{de, ser, Deserialize, Serialize};

    /// JSON serialization
    pub fn serialize<S>(uint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        uint.to_string().serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        Ok(BigUint::from_str(v.as_str()).map_err(|e| de::Error::custom(e.to_string()))?)
    }
}

const SIZE_UNITS: [&str; 8] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];

/// Convert BigUint into size mod, like "0 B", "1.95 KiB" and "5 MiB", etc...
pub fn biguint_size_str(size: &BigUint) -> String {
    let mut size = size.clone();
    let mut i = 0;
    let mut decimal = BigUint::zero();
    let unit = BigUint::from(1024_u64);
    let mask = BigUint::from(1023_u64);
    while size >= unit && i + 1 < SIZE_UNITS.len() {
        decimal = size.clone() & mask.clone();
        size >>= 10;
        i += 1;
    }
    if decimal.is_zero() {
        format!("{} {}", size, SIZE_UNITS[i])
    } else {
        let size = size.to_f64().unwrap();
        let part = decimal.to_f64().unwrap();
        let out = part / 1024_f64 + size;
        format!("{:0.3} {}", out, SIZE_UNITS[i])
    }
}

#[test]
fn test_size_str() {
    let cases = vec![
        (0_u128, "0 B"),
        (1, "1 B"),
        (1024, "1 KiB"),
        (2000, "1.953 KiB"),
        (5 << 20, "5 MiB"),
        (11 << 60, "11 EiB"),
    ];

    for (num, expect) in cases {
        let big_uint = BigUint::from(num);
        let size = biguint_size_str(&big_uint);
        assert_eq!(size, expect);
    }

    let mut big_uint = BigUint::from(50000_u64);
    big_uint <<= 70;
    assert_eq!(biguint_size_str(&big_uint), "50000 ZiB");
}

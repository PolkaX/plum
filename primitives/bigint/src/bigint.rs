// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use num_bigint::{BigInt, Sign};
use num_traits::{Signed, ToPrimitive, Zero};
use serde::{de, ser};

/// A BigInt wrapper that implement CBOR and JSON serialization/deserialization.
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigIntWrapper(BigInt);

impl BigIntWrapper {
    /// Consumes the wrapper, returning the underlying BigInt.
    pub fn into_inner(self) -> BigInt {
        self.0
    }

    /// Don't consume the wrapper, borrowing the underlying BigInt.
    pub fn as_inner(&self) -> &BigInt {
        &self.0
    }

    /// Don't consume the wrapper, mutable borrowing the underlying BigInt.
    pub fn as_mut_inner(&mut self) -> &mut BigInt {
        &mut self.0
    }
}

impl AsRef<BigInt> for BigIntWrapper {
    fn as_ref(&self) -> &BigInt {
        self.as_inner()
    }
}

impl AsMut<BigInt> for BigIntWrapper {
    fn as_mut(&mut self) -> &mut BigInt {
        self.as_mut_inner()
    }
}

impl From<BigInt> for BigIntWrapper {
    fn from(int: BigInt) -> Self {
        Self(int)
    }
}

// Implement CBOR serialization for BigIntWrapper.
impl encode::Encode for BigIntWrapper {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        let (sign, mut v) = self.0.to_bytes_be();
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
        e.bytes(&v)?.ok()
    }
}

// Implement CBOR deserialization for BigIntWrapper.
impl<'b> decode::Decode<'b> for BigIntWrapper {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let v = d.bytes()?.to_vec();
        if v.is_empty() {
            return Ok(BigIntWrapper::default());
        }
        let sign = match &v[0] {
            0 => Sign::Plus,
            1 => Sign::Minus,
            _ => {
                return Err(decode::Error::Message(
                    "big int prefix should be either 0 or 1",
                ));
            }
        };
        Ok(BigIntWrapper(BigInt::from_bytes_be(sign, &v[1..])))
    }
}

// Implement JSON serialization for BigIntWrapper.
impl ser::Serialize for BigIntWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::json::serialize(self.as_inner(), serializer)
    }
}

// Implement JSON deserialization for BigIntWrapper.
impl<'de> de::Deserialize<'de> for BigIntWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Self(self::json::deserialize(deserializer)?))
    }
}

/// A BigInt reference wrapper that implement CBOR and JSON serialization.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigIntRefWrapper<'a>(&'a BigInt);

impl<'a> BigIntRefWrapper<'a> {
    /// Don't consume the wrapper, borrowing the underlying BigInt.
    pub fn as_inner(&self) -> &BigInt {
        self.0
    }
}

impl<'a> AsRef<BigInt> for BigIntRefWrapper<'a> {
    fn as_ref(&self) -> &BigInt {
        self.as_inner()
    }
}

impl<'a> From<&'a BigInt> for BigIntRefWrapper<'a> {
    fn from(int: &'a BigInt) -> Self {
        Self(int)
    }
}

// Implement CBOR serialization for BigIntRefWrapper.
impl<'a> encode::Encode for BigIntRefWrapper<'a> {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        let (sign, mut v) = self.0.to_bytes_be();
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
        e.bytes(&v)?.ok()
    }
}

// Implement JSON serialization for BigIntRefWrapper.
impl<'a> ser::Serialize for BigIntRefWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::json::serialize(self.as_inner(), serializer)
    }
}

/// JSON serialization/deserialization
pub mod json {
    use num_bigint::BigInt;
    use serde::{de, ser, Deserialize, Serialize};

    /// JSON serialization
    pub fn serialize<S>(int: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        int.to_string().serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bigint = String::deserialize(deserializer)?
            .parse::<BigInt>()
            .map_err(|e| de::Error::custom(e.to_string()))?;
        Ok(bigint)
    }
}

const SIZE_UNITS: [&str; 8] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];

/// Convert BigInt into size mod, like "0 B", "1.95 KiB" and "5 MiB", etc...
pub fn bigint_size_str(size: &BigInt) -> String {
    let (sign, mut size) = (size.sign(), size.abs());
    let mut i = 0;
    let mut decimal = BigInt::zero();
    let unit = BigInt::from(1024_u64);
    let mask = BigInt::from(1023_u64);
    while size >= unit && i + 1 < SIZE_UNITS.len() {
        decimal = size.clone() & mask.clone();
        size >>= 10;
        i += 1;
    }
    if decimal.is_zero() {
        if sign == Sign::Minus {
            format!("-{} {}", size, SIZE_UNITS[i])
        } else {
            format!("{} {}", size, SIZE_UNITS[i])
        }
    } else {
        let size = size.to_f64().unwrap();
        let part = decimal.to_f64().unwrap();
        let out = part / 1024_f64 + size;
        if sign == Sign::Minus {
            format!("-{:0.3} {}", out, SIZE_UNITS[i])
        } else {
            format!("{:0.3} {}", out, SIZE_UNITS[i])
        }
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
        let big_int = BigInt::from(num);
        let size = bigint_size_str(&big_int);
        assert_eq!(size, expect);
    }

    let mut big_int = BigInt::from(50000_u64);
    big_int <<= 70;
    assert_eq!(bigint_size_str(&big_int), "50000 ZiB");

    let cases = vec![
        (0_i128, "0 B"),
        (-1, "-1 B"),
        (-1024, "-1 KiB"),
        (-2000, "-1.953 KiB"),
        (-5 << 20, "-5 MiB"),
        (-11 << 60, "-11 EiB"),
    ];
    for (num, expect) in cases {
        let big_int = BigInt::from(num);
        let size = bigint_size_str(&big_int);
        assert_eq!(size, expect);
    }

    let mut big_int = BigInt::from(-50000_i64);
    big_int <<= 70;
    assert_eq!(bigint_size_str(&big_int), "-50000 ZiB");
}

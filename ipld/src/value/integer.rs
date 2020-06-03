// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;

use minicbor::{data::Type, decode, encode, Decoder, Encoder};
use serde::{de, ser};

/// The Integer kind of IPLD Data Model.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Integer(i128);

impl Integer {
    /// Convert self into inner i128.
    pub fn into_inner(self) -> i128 {
        self.0
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<u8> for Integer {
    fn from(u: u8) -> Self {
        Integer(i128::from(u))
    }
}

impl From<u16> for Integer {
    fn from(u: u16) -> Self {
        Integer(i128::from(u))
    }
}

impl From<u32> for Integer {
    fn from(u: u32) -> Self {
        Integer(i128::from(u))
    }
}

impl From<u64> for Integer {
    fn from(u: u64) -> Self {
        Integer(i128::from(u))
    }
}

impl From<i8> for Integer {
    fn from(i: i8) -> Self {
        Integer(i128::from(i))
    }
}

impl From<i16> for Integer {
    fn from(i: i16) -> Self {
        Integer(i128::from(i))
    }
}

impl From<i32> for Integer {
    fn from(i: i32) -> Self {
        Integer(i128::from(i))
    }
}

impl From<i64> for Integer {
    fn from(i: i64) -> Self {
        Integer(i128::from(i))
    }
}

impl From<i128> for Integer {
    fn from(i: i128) -> Self {
        Integer(i)
    }
}

// Integer encoding must be as short as possible.
// See [Strictness](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md#strictness) for details.
// Implement CBOR serialization for Integer.
impl encode::Encode for Integer {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        let integer = self.0;
        if integer < 0 {
            if integer >= i128::from(i8::min_value()) {
                e.i8(integer as i8)?.ok()
            } else if integer >= i128::from(i16::min_value()) {
                e.i16(integer as i16)?.ok()
            } else if integer >= i128::from(i32::min_value()) {
                e.i32(integer as i32)?.ok()
            } else if integer >= i128::from(i64::min_value()) {
                e.i64(integer as i64)?.ok()
            } else {
                panic!("The integer can't be stored in CBOR");
            }
        } else {
            // integer >= 0
            if integer <= i128::from(u8::max_value()) {
                e.u8(integer as u8)?.ok()
            } else if integer <= i128::from(u16::max_value()) {
                e.u16(integer as u16)?.ok()
            } else if integer <= i128::from(u32::max_value()) {
                e.u32(integer as u32)?.ok()
            } else if integer <= i128::from(u64::max_value()) {
                e.u64(integer as u64)?.ok()
            } else {
                panic!("The integer can't be stored in CBOR");
            }
        }
    }
}

// Implement CBOR deserialization for Integer.
impl<'b> decode::Decode<'b> for Integer {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let integer = match d.datatype()? {
            Type::U8 => Integer(i128::from(d.u8()?)),
            Type::U16 => Integer(i128::from(d.u16()?)),
            Type::U32 => Integer(i128::from(d.u32()?)),
            Type::U64 => Integer(i128::from(d.u64()?)),
            Type::I8 => Integer(i128::from(d.i8()?)),
            Type::I16 => Integer(i128::from(d.i16()?)),
            Type::I32 => Integer(i128::from(d.i32()?)),
            Type::I64 => Integer(i128::from(d.i64()?)),
            _ => unreachable!("expect integer type"),
        };
        Ok(integer)
    }
}

// Implement JSON serialization for Integer.
impl ser::Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_i128(self.0)
    }
}

// Implement JSON deserialization for Integer.
impl<'de> de::Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        /// Integer visitor for generating IPLD from JSON
        struct IntegerVisitor;
        impl<'de> de::Visitor<'de> for IntegerVisitor {
            type Value = Integer;

            #[inline]
            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("any valid JSON number")
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Integer::from(v))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Integer::from(v))
            }

            #[inline]
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Integer::from(v))
            }
        }

        deserializer.deserialize_any(IntegerVisitor)
    }
}

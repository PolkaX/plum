// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! A Wrapper of `Vec<u8>` with the specific CBOR and JSON serialization/deserialization,
//! in order to be compatible with golang standard library.
//! Fuck golang standard library.

#![deny(missing_docs)]

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser, Deserialize, Serialize};

/// A wrapper of Vec<u8> that implement CBOR and JSON serialization/deserialization.
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash, Default)]
pub struct Bytes(Vec<u8>);

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.as_inner()
    }
}

impl AsMut<[u8]> for Bytes {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_inner()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Bytes {
    /// Consumes the wrapper, returning the underlying Vec<u8>.
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Don't consume the wrapper, borrowing the underlying Vec<u8>.
    pub fn as_inner(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Don't consume the wrapper, mutable borrowing the underlying Vec<u8>.
    pub fn as_mut_inner(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
}

// Implement CBOR serialization for Bytes.
impl encode::Encode for Bytes {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self.as_inner())?.ok()
    }
}

// Implement CBOR deserialization for Bytes.
impl<'b> decode::Decode<'b> for Bytes {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        Ok(Bytes(d.bytes()?.to_vec()))
    }
}

/// Implement JSON serialization of Vec<u8> using base64.
impl ser::Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::serialize(self.as_inner(), serializer)
    }
}

/// Implement JSON deserialization of Vec<u8> using base64.
impl<'de> de::Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Self(self::deserialize(deserializer)?))
    }
}

/// A wrapper of &[u8] that implement CBOR and JSON serialization.
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash, Default)]
pub struct BytesRef<'a>(&'a [u8]);

impl<'a> BytesRef<'a> {
    /// Don't consume the wrapper, borrowing the underlying &[u8].
    pub fn as_inner(&self) -> &[u8] {
        self.0
    }
}

impl<'a> AsRef<[u8]> for BytesRef<'a> {
    fn as_ref(&self) -> &[u8] {
        self.as_inner()
    }
}

impl<'a> From<&'a [u8]> for BytesRef<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

// Implement CBOR serialization for BytesRef.
impl<'a> encode::Encode for BytesRef<'a> {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self.as_inner())?.ok()
    }
}

/// Implement JSON serialization of &[u8] using base64.
impl<'a> ser::Serialize for BytesRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::serialize(self.as_inner(), serializer)
    }
}

/// Implement JSON serialization of Vec<u8> using base64.
pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    base64::encode(bytes).serialize(serializer)
}

/// Implement JSON deserialization of Vec<u8> using base64.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    base64::decode(String::deserialize(deserializer)?)
        .map_err(|err| de::Error::custom(format!("base64 decode error: {}", err)))
}

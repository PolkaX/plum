// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::str::FromStr;

use fixed_hash::construct_fixed_hash;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser};

construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    pub struct H256(32);
}

impl FromStr for H256 {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        if bytes.len() != Self::len_bytes() {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        Ok(Self::from_slice(&bytes))
    }
}

// Implement JSON serialization for H256.
impl ser::Serialize for H256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        hex::encode(self.as_bytes()).serialize(serializer)
    }
}

// Implement JSON deserialization for H256.
impl<'de> de::Deserialize<'de> for H256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes = hex::decode(String::deserialize(deserializer)?)
            .map_err(|err| de::Error::custom(format!("hex decode error: {}", err)))?;
        if bytes.len() == H256::len_bytes() {
            Ok(H256::from_slice(bytes.as_slice()))
        } else {
            Err(de::Error::custom("H256 length must be 32 Bytes"))
        }
    }
}

// Implement CBOR serialization for H256.
impl encode::Encode for H256 {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self.as_bytes())?.ok()
    }
}

// Implement CBOR deserialization for H256.
impl<'b> decode::Decode<'b> for H256 {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        if bytes.len() == H256::len_bytes() {
            Ok(H256::from_slice(bytes))
        } else {
            Err(decode::Error::Message("H256 length must be 32 Bytes"))
        }
    }
}

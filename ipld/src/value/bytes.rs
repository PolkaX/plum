// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser};

const SLASH: &str = "/";
const BYTES: &str = "bytes";

/// The Bytes kind of IPLD Data Model.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Convert self into inner string.
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = multibase::encode(multibase::Base::Base64, &self.0);
        write!(f, "{}", output)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes(bytes)
    }
}

impl From<&[u8]> for Bytes {
    fn from(bytes: &[u8]) -> Self {
        Bytes(bytes.to_owned())
    }
}

// Implement CBOR serialization for Bytes.
impl encode::Encode for Bytes {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self.0)?.ok()
    }
}

// Implement CBOR deserialization for Bytes.
impl<'b> decode::Decode<'b> for Bytes {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let s = d.bytes()?.to_owned();
        Ok(Bytes(s))
    }
}

// See [DAG-JSON](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-json.md#bytes-kind) for details.
// Implement JSON serialization for Bytes.
impl ser::Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = serde_json::json!({SLASH: {BYTES: self.to_string()}});
        value.serialize(serializer)
    }
}

// Implement JSON deserialization for Bytes.
impl<'de> de::Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use serde_json::{Map, Value};

        // JSON Object represents IPLD Bytes if it is `{ "/": {"bytes": "..."} }`
        let map = Map::<String, Value>::deserialize(deserializer)?;

        if map.len() == 1 {
            if let Some(Value::Object(map)) = map.get(SLASH) {
                if map.len() == 1 {
                    if let Some(Value::String(bytes)) = map.get(BYTES) {
                        let (base, bytes) = multibase::decode(bytes)
                            .map_err(|e| de::Error::custom(e.to_string()))?;
                        if base != multibase::Base::Base64 {
                            return Err(de::Error::custom("unexpected multibase algorithm"));
                        }
                        return Ok(Bytes(bytes));
                    }
                }
            }
        }
        Err(de::Error::custom(
            "unexpected JSON object for IPLD Bytes kind",
        ))
    }
}

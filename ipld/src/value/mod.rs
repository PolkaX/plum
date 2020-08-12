// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

#[macro_use]
mod macros;

mod bytes;
mod integer;
mod map;

pub use self::bytes::Bytes;
pub use self::integer::Integer;
pub use self::map::{Map, MapKey};

use std::collections::BTreeMap;

use cid::Cid;
use minicbor::{data::Type, decode, encode, Decoder, Encoder};
use serde::{de, ser};

/// The [IPLD Data Model](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    /// Null value.
    ///
    /// ```
    /// # use ipld::{ipld, Value};
    /// assert_eq!(Value::Null, ipld!(null));
    /// ```
    Null,
    /// Boolean value.
    ///
    /// ```
    /// # use ipld::{ipld, Value};
    /// assert_eq!(Value::Bool(true), ipld!(true));
    /// assert_eq!(Value::Bool(false), ipld!(false));
    /// ```
    Bool(bool),
    /// Integer value.
    ///
    /// ```
    /// # use ipld::{ipld, Value, Integer};
    /// assert_eq!(Value::Integer(Integer::from(123)), ipld!(123));
    /// assert_eq!(Value::Integer(Integer::from(-123)), ipld!(-123));
    /// ```
    Integer(Integer),
    /// Floating point value.
    ///
    /// ```
    /// # use ipld::{ipld, Value};
    /// assert_eq!(Value::Float(123.0), ipld!(123.0));
    /// assert_eq!(Value::Float(-123.0), ipld!(-123.0));
    /// ```
    Float(f64),
    /// UTF-8 string value.
    ///
    /// ```
    /// # use ipld::{ipld, Value};
    /// assert_eq!(Value::String("string".into()), ipld!("string"));
    /// ```
    String(String),
    /// Byte string value.
    ///
    /// ```
    /// # use ipld::{ipld, Value};
    /// assert_eq!(Value::Bytes(vec![0, 1, 100, 255].into()), ipld!(bytes![0, 1, 100, 255]));
    /// assert_eq!(Value::Bytes(vec![100; 3].into()), ipld!(bytes![100; 3]));
    /// assert_eq!(Value::Bytes(vec![100; 3].into()), ipld!(bytes![100, 100, 100]));
    /// ```
    Bytes(Bytes),
    /// List value.
    ///
    /// ```
    /// # use cid::Cid;
    /// # use ipld::{ipld, Value};
    /// # use std::collections::BTreeMap;
    /// assert_eq!(
    ///     Value::List(vec![
    ///         Value::Null,
    ///         Value::Bool(true),
    ///         Value::Integer(123.into()),
    ///         Value::Float(123.0),
    ///         Value::String("string".into()),
    ///         Value::Bytes(vec![1, 255].into()),
    ///         Value::List(vec![]),
    ///         Value::Map(BTreeMap::new()),
    ///         Value::Link("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL".parse::<Cid>().unwrap()),
    ///     ]),
    ///     ipld!([null, true, 123, 123.0, "string", bytes![1, 255], [], {}, link!("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL")]),
    /// );
    /// ```
    List(Vec<Value>),
    /// Map value.
    ///
    /// ```
    /// # use cid::Cid;
    /// # use ipld::{ipld, Value, MapKey};
    /// # use std::collections::BTreeMap;
    /// let mut map = BTreeMap::<MapKey, Value>::new();
    /// map.insert("null".into(), Value::Null);
    /// map.insert("bool".into(), Value::Bool(true));
    /// map.insert("integer".into(), Value::Integer(123.into()));
    /// map.insert("float".into(), Value::Float(123.0));
    /// map.insert("string".into(), Value::String("string".into()));
    /// map.insert("bytes".into(), Value::Bytes(vec![1, 255].into()));
    /// map.insert("list".into(), Value::List(vec![]));
    /// map.insert("map".into(), Value::Map(BTreeMap::new()));
    /// map.insert("link".into(), Value::Link("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL".parse::<Cid>().unwrap()));
    /// assert_eq!(
    ///     Value::Map(map),
    ///     ipld!({
    ///         "null":null,
    ///         "bool":true,
    ///         "integer":123,
    ///         "float":123.0,
    ///         "string":"string",
    ///         "bytes":bytes![1, 255],
    ///         "list":[],
    ///         "map":{},
    ///         "link":link!("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL"),
    ///     }),
    /// );
    /// ```
    Map(BTreeMap<MapKey, Value>),
    /// Link value.
    ///
    /// ```
    /// # use cid::Cid;
    /// # use ipld::{ipld, Value};
    /// let cid = "QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL".parse::<Cid>().unwrap();
    /// assert_eq!(Value::Link(cid), ipld!(link!("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL")));
    /// ```
    Link(Cid),
}

// See [DAG-CBOR](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md) for details.
// Implement CBOR serialization for Value.
impl encode::Encode for Value {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        match self {
            Value::Null => e.null()?.ok(),
            Value::Bool(bool) => e.bool(*bool)?.ok(),
            // Integer encoding must be as short as possible.
            Value::Integer(integer) => e.encode(integer)?.ok(),
            // FIXME: Strict floating point encoding rules need to be resolved.
            // Current CBOR encoding implementations used by IPLD libraries are not unified in their approach.
            Value::Float(f64) => e.f64(*f64)?.ok(),
            Value::Bytes(bytes) => e.encode(bytes)?.ok(),
            Value::String(string) => e.str(string)?.ok(),
            Value::List(list) => e.encode(list)?.ok(),
            Value::Map(map) => e.encode(map)?.ok(),
            Value::Link(cid) => e.encode(cid)?.ok(),
        }
    }
}

// Implement CBOR deserialization for Value.
impl<'b> decode::Decode<'b> for Value {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        match d.datatype()? {
            Type::Null => {
                d.skip()?;
                Ok(Value::Null)
            }
            Type::Bool => Ok(Value::Bool(d.bool()?)),
            Type::U8 | Type::U16 | Type::U32 | Type::U64 => {
                Ok(Value::Integer(d.decode::<Integer>()?))
            }
            Type::I8 | Type::I16 | Type::I32 | Type::I64 => {
                Ok(Value::Integer(d.decode::<Integer>()?))
            }
            Type::F16 => Ok(Value::Float(f64::from(d.f16()?))),
            Type::F32 => Ok(Value::Float(f64::from(d.f32()?))),
            Type::F64 => Ok(Value::Float(d.f64()?)),
            Type::Bytes => Ok(Value::Bytes(d.decode::<Bytes>()?)),
            Type::String => Ok(Value::String(d.str()?.to_owned())),
            Type::Array => Ok(Value::List(d.decode::<Vec<Value>>()?)),
            Type::Map => Ok(Value::Map(d.decode::<BTreeMap<MapKey, Value>>()?)),
            Type::Tag => Ok(Value::Link(d.decode::<Cid>()?)),
            Type::Break | Type::Unknown(_) | Type::Undefined | Type::Simple => {
                Err(decode::Error::Message("unexpected type"))
            }
        }
    }
}

// See [DAG-JSON](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-json.md) for details.
// Implement JSON serialization for Value.
impl ser::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Bool(bool) => serializer.serialize_bool(*bool),
            Value::Integer(integer) => integer.serialize(serializer),
            Value::Float(f64) => serializer.serialize_f64(*f64),
            Value::String(string) => serializer.serialize_str(string),
            // The Bytes kind is represented as an object with "bytes" as key and a Multibase Base64 encoded string as value.
            Value::Bytes(bytes) => bytes.serialize(serializer),
            Value::List(list) => list.serialize(serializer),
            Value::Map(map) => map.serialize(serializer),
            Value::Link(link) => link.serialize(serializer),
        }
    }
}

// Implement JSON deserialization for Value.
impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonVisitor)
    }
}

struct JsonVisitor;
impl<'de> de::Visitor<'de> for JsonVisitor {
    type Value = Value;

    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("any valid JSON value")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bool(v))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Integer(v.into()))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Integer(v.into()))
    }

    #[inline]
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Integer(v.into()))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Float(v))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(String::from(value))
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_unit()
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(Value::List(vec))
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        const SLASH: &str = "/";
        const BYTES: &str = "bytes";

        let mut map = BTreeMap::new();

        while let Some((key, value)) = visitor.next_entry()? {
            map.insert(key, value);
        }

        if map.len() == 1 {
            if let Some(value) = map.get(SLASH) {
                match value {
                    // JSON Object represents IPLD Link if it is `{ "/": "..." }`
                    Value::String(string) => {
                        return Ok(Value::Link(string.parse().map_err(de::Error::custom)?));
                    }
                    // JSON Object represents IPLD Bytes if it is `{ "/": { "bytes": "..." } }`
                    Value::Map(map) => {
                        if map.len() == 1 {
                            if let Some(Value::String(string)) = map.get(BYTES) {
                                let (base, bytes) = multibase::decode(string)
                                    .map_err(|e| de::Error::custom(e.to_string()))?;
                                if base != multibase::Base::Base64 {
                                    return Err(de::Error::custom(
                                        "unexpected multibase algorithm",
                                    ));
                                }
                                return Ok(Value::Bytes(bytes.into()));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Value::Map(map))
    }
}

#[test]
fn test_ipld_value_cbor_and_json() {
    const TEST_OBJ_ROOT: &str = "tests/test_objects/";

    let content = std::fs::read_to_string(format!("{}expected.json", TEST_OBJ_ROOT)).unwrap();
    let value = serde_json::from_str::<Value>(&content).unwrap();
    match value {
        Value::Map(map) => {
            for (key, _value) in map {
                let json_file_name = format!("{}{}.json", TEST_OBJ_ROOT, key);
                let json = std::fs::read_to_string(json_file_name).unwrap();
                let json_value = serde_json::from_str::<Value>(&json).unwrap();
                let cbor_file_name = format!("{}{}.cbor", TEST_OBJ_ROOT, key);
                let cbor = std::fs::read(cbor_file_name).unwrap();
                let cbor_value = minicbor::decode::<Value>(&cbor).unwrap();
                assert_eq!(json_value, cbor_value);
            }
        }
        _ => panic!(),
    }
}

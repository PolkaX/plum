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
pub enum IpldValue {
    /// Null value.
    ///
    /// ```
    /// # use plum_ipld::{ipld, IpldValue};
    /// assert_eq!(IpldValue::Null, ipld!(null));
    /// ```
    Null,
    /// Boolean value.
    ///
    /// ```
    /// # use plum_ipld::{ipld, IpldValue};
    /// assert_eq!(IpldValue::Bool(true), ipld!(true));
    /// assert_eq!(IpldValue::Bool(false), ipld!(false));
    /// ```
    Bool(bool),
    /// Integer value.
    ///
    /// ```
    /// # use plum_ipld::{ipld, IpldValue, Integer};
    /// assert_eq!(IpldValue::Integer(Integer::from(123)), ipld!(123));
    /// assert_eq!(IpldValue::Integer(Integer::from(-123)), ipld!(-123));
    /// ```
    Integer(Integer),
    /// Floating point value.
    ///
    /// ```
    /// # use plum_ipld::{ipld, IpldValue};
    /// assert_eq!(IpldValue::Float(123.0), ipld!(123.0));
    /// assert_eq!(IpldValue::Float(-123.0), ipld!(-123.0));
    /// ```
    Float(f64),
    /// UTF-8 string value.
    ///
    /// ```
    /// # use plum_ipld::{ipld, IpldValue};
    /// assert_eq!(IpldValue::String("string".into()), ipld!("string"));
    /// ```
    String(String),
    /// Byte string value.
    ///
    /// ```
    /// # use plum_ipld::{ipld, IpldValue};
    /// assert_eq!(IpldValue::Bytes(vec![0, 1, 100, 255].into()), ipld!(bytes![0, 1, 100, 255]));
    /// assert_eq!(IpldValue::Bytes(vec![100; 3].into()), ipld!(bytes![100; 3]));
    /// assert_eq!(IpldValue::Bytes(vec![100; 3].into()), ipld!(bytes![100, 100, 100]));
    /// ```
    Bytes(Bytes),
    /// List value.
    ///
    /// ```
    /// # use cid::Cid;
    /// # use plum_ipld::{ipld, IpldValue};
    /// # use std::collections::BTreeMap;
    /// assert_eq!(
    ///     IpldValue::List(vec![
    ///         IpldValue::Null,
    ///         IpldValue::Bool(true),
    ///         IpldValue::Integer(123.into()),
    ///         IpldValue::Float(123.0),
    ///         IpldValue::String("string".into()),
    ///         IpldValue::Bytes(vec![1, 255].into()),
    ///         IpldValue::List(vec![]),
    ///         IpldValue::Map(BTreeMap::new()),
    ///         IpldValue::Link("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL".parse::<Cid>().unwrap()),
    ///     ]),
    ///     ipld!([null, true, 123, 123.0, "string", bytes![1, 255], [], {}, link!("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL")]),
    /// );
    /// ```
    List(Vec<IpldValue>),
    /// Map value.
    ///
    /// ```
    /// # use cid::Cid;
    /// # use plum_ipld::{ipld, IpldValue, MapKey};
    /// # use std::collections::BTreeMap;
    /// let mut map = BTreeMap::<MapKey, IpldValue>::new();
    /// map.insert("null".into(), IpldValue::Null);
    /// map.insert("bool".into(), IpldValue::Bool(true));
    /// map.insert("integer".into(), IpldValue::Integer(123.into()));
    /// map.insert("float".into(), IpldValue::Float(123.0));
    /// map.insert("string".into(), IpldValue::String("string".into()));
    /// map.insert("bytes".into(), IpldValue::Bytes(vec![1, 255].into()));
    /// map.insert("list".into(), IpldValue::List(vec![]));
    /// map.insert("map".into(), IpldValue::Map(BTreeMap::new()));
    /// map.insert("link".into(), IpldValue::Link("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL".parse::<Cid>().unwrap()));
    /// assert_eq!(
    ///     IpldValue::Map(map),
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
    Map(BTreeMap<MapKey, IpldValue>),
    /// Link value.
    ///
    /// ```
    /// # use cid::Cid;
    /// # use plum_ipld::{ipld, IpldValue};
    /// let cid = "QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL".parse::<Cid>().unwrap();
    /// assert_eq!(IpldValue::Link(cid), ipld!(link!("QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL")));
    /// ```
    Link(Cid),
}

// See [DAG-CBOR](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md) for details.
// Implement CBOR serialization for IpldValue.
impl encode::Encode for IpldValue {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        match self {
            IpldValue::Null => e.null()?.ok(),
            IpldValue::Bool(bool) => e.bool(*bool)?.ok(),
            // Integer encoding must be as short as possible.
            IpldValue::Integer(integer) => e.encode(integer)?.ok(),
            // FIXME: Strict floating point encoding rules need to be resolved.
            // Current CBOR encoding implementations used by IPLD libraries are not unified in their approach.
            IpldValue::Float(f64) => e.f64(*f64)?.ok(),
            IpldValue::Bytes(bytes) => e.encode(bytes)?.ok(),
            IpldValue::String(string) => e.str(string)?.ok(),
            IpldValue::List(list) => e.encode(list)?.ok(),
            IpldValue::Map(map) => e.encode(map)?.ok(),
            IpldValue::Link(cid) => e.encode(cid)?.ok(),
        }
    }
}

// Implement CBOR deserialization for IpldValue.
impl<'b> decode::Decode<'b> for IpldValue {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        match d.datatype()? {
            Type::Null => {
                d.skip()?;
                Ok(IpldValue::Null)
            }
            Type::Bool => Ok(IpldValue::Bool(d.bool()?)),
            Type::U8 | Type::U16 | Type::U32 | Type::U64 => {
                Ok(IpldValue::Integer(d.decode::<Integer>()?))
            }
            Type::I8 | Type::I16 | Type::I32 | Type::I64 => {
                Ok(IpldValue::Integer(d.decode::<Integer>()?))
            }
            Type::F16 => Ok(IpldValue::Float(f64::from(d.f16()?))),
            Type::F32 => Ok(IpldValue::Float(f64::from(d.f32()?))),
            Type::F64 => Ok(IpldValue::Float(d.f64()?)),
            Type::Bytes => Ok(IpldValue::Bytes(d.decode::<Bytes>()?)),
            Type::String => Ok(IpldValue::String(d.str()?.to_owned())),
            Type::Array => Ok(IpldValue::List(d.decode::<Vec<IpldValue>>()?)),
            Type::Map => Ok(IpldValue::Map(d.decode::<BTreeMap<MapKey, IpldValue>>()?)),
            Type::Tag => Ok(IpldValue::Link(d.decode::<Cid>()?)),
            Type::Break | Type::Unknown(_) | Type::Undefined | Type::Simple => {
                Err(decode::Error::Message("unexpected type"))
            }
        }
    }
}

// See [DAG-JSON](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-json.md) for details.
// Implement JSON serialization for IpldValue.
impl ser::Serialize for IpldValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            IpldValue::Null => serializer.serialize_none(),
            IpldValue::Bool(bool) => serializer.serialize_bool(*bool),
            IpldValue::Integer(integer) => integer.serialize(serializer),
            IpldValue::Float(f64) => serializer.serialize_f64(*f64),
            IpldValue::String(string) => serializer.serialize_str(string),
            // The Bytes kind is represented as an object with "bytes" as key and a Multibase Base64 encoded string as value.
            IpldValue::Bytes(bytes) => bytes.serialize(serializer),
            IpldValue::List(list) => list.serialize(serializer),
            IpldValue::Map(map) => map.serialize(serializer),
            IpldValue::Link(link) => link.serialize(serializer),
        }
    }
}

// Implement JSON deserialization for IpldValue.
impl<'de> de::Deserialize<'de> for IpldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonVisitor)
    }
}

struct JsonVisitor;
impl<'de> de::Visitor<'de> for JsonVisitor {
    type Value = IpldValue;

    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("any valid JSON value")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Bool(v))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Integer(v.into()))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Integer(v.into()))
    }

    #[inline]
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Integer(v.into()))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Float(v))
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
        Ok(IpldValue::String(value))
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
        Ok(IpldValue::Null)
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

        Ok(IpldValue::List(vec))
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
                    IpldValue::String(string) => {
                        return Ok(IpldValue::Link(string.parse().map_err(de::Error::custom)?));
                    }
                    // JSON Object represents IPLD Bytes if it is `{ "/": { "bytes": "..." } }`
                    IpldValue::Map(map) => {
                        if map.len() == 1 {
                            if let Some(IpldValue::String(string)) = map.get(BYTES) {
                                let (base, bytes) = multibase::decode(string)
                                    .map_err(|e| de::Error::custom(e.to_string()))?;
                                if base != multibase::Base::Base64 {
                                    return Err(de::Error::custom(
                                        "unexpected multibase algorithm",
                                    ));
                                }
                                return Ok(IpldValue::Bytes(bytes.into()));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(IpldValue::Map(map))
    }
}

#[test]
fn test_ipld_value_cbor_and_json() {
    const TEST_OBJ_ROOT: &str = "tests/test_objects/";

    let content = std::fs::read_to_string(format!("{}expected.json", TEST_OBJ_ROOT)).unwrap();
    let value = serde_json::from_str::<IpldValue>(&content).unwrap();
    match value {
        IpldValue::Map(map) => {
            for (key, _value) in map {
                let json_file_name = format!("{}{}.json", TEST_OBJ_ROOT, key);
                let json = std::fs::read_to_string(json_file_name).unwrap();
                let json_value = serde_json::from_str::<IpldValue>(&json).unwrap();
                let cbor_file_name = format!("{}{}.cbor", TEST_OBJ_ROOT, key);
                let cbor = std::fs::read(cbor_file_name).unwrap();
                let cbor_value = minicbor::decode::<IpldValue>(&cbor).unwrap();
                assert_eq!(json_value, cbor_value);
            }
        }
        _ => panic!(),
    }
}

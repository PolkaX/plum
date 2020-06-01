// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;

use serde::{
    de,
    ser::{self, SerializeMap, SerializeSeq},
};

use super::*;

// Implement JSON serialization for IpldValue.
impl ser::Serialize for IpldValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            IpldValue::Null => serializer.serialize_none(),
            IpldValue::Bool(bool) => serializer.serialize_bool(*bool),
            IpldValue::Integer(i128) => {
                if *i128 > 0 {
                    serializer.serialize_u64(*i128 as u64)
                } else {
                    serializer.serialize_i64(*i128 as i64)
                }
            }
            IpldValue::Float(f64) => serializer.serialize_f64(*f64),
            IpldValue::String(string) => serializer.serialize_str(string),
            IpldValue::Bytes(bytes) => serializer.serialize_bytes(bytes),
            IpldValue::List(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for element in list {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            IpldValue::Map(map) => {
                let mut m = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    m.serialize_entry(k, v)?;
                }
                m.end()
            }
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
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_byte_buf(v.to_owned())
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Bytes(v))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Integer(v.into()))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
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
        Ok(IpldValue::Integer(v))
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Bool(v))
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
        const LINK_KEY: &str = "/";

        let mut values = Vec::<(String, IpldValue)>::new();

        while let Some((key, value)) = visitor.next_entry()? {
            values.push((key, value));
        }

        // JSON Object represents IPLD Link if it is `{ "/": "...." }`
        if let Some((key, IpldValue::String(value))) = values.first() {
            if key.as_str() == LINK_KEY && values.len() == 1 {
                let cid = Cid::try_from(value.as_str()).map_err(de::Error::custom)?;
                return Ok(IpldValue::Link(cid));
            }
        }

        let values = values.into_iter().collect::<BTreeMap<String, IpldValue>>();
        Ok(IpldValue::Map(values))
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IpldValue::Float(v))
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{data::Type, decode, encode, Decoder, Encoder};

use super::*;

// Implement CBOR serialization for IpldValue.
impl encode::Encode for IpldValue {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        match self {
            IpldValue::Null => e.null()?.ok(),
            IpldValue::Bool(bool) => e.bool(*bool)?.ok(),
            IpldValue::Integer(i128) => e.i64(*i128 as i64)?.ok(),
            IpldValue::Float(f64) => e.f64(*f64)?.ok(),
            IpldValue::Bytes(bytes) => e.bytes(bytes)?.ok(),
            IpldValue::String(string) => e.str(string)?.ok(),
            IpldValue::List(list) => {
                let e = e.array(list.len() as u64)?;
                for value in list {
                    e.encode(value)?;
                }
                e.ok()
            }
            IpldValue::Map(map) => {
                let e = e.map(map.len() as u64)?;
                for (key, value) in map {
                    e.encode(key)?.encode(value)?;
                }
                e.ok()
            }
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
            Type::U8 => Ok(IpldValue::Integer(i128::from(d.u8()?))),
            Type::U16 => Ok(IpldValue::Integer(i128::from(d.u16()?))),
            Type::U32 => Ok(IpldValue::Integer(i128::from(d.u32()?))),
            Type::U64 => Ok(IpldValue::Integer(i128::from(d.u32()?))),
            Type::I8 => Ok(IpldValue::Integer(i128::from(d.i8()?))),
            Type::I16 => Ok(IpldValue::Integer(i128::from(d.i16()?))),
            Type::I32 => Ok(IpldValue::Integer(i128::from(d.i32()?))),
            Type::I64 => Ok(IpldValue::Integer(i128::from(d.i64()?))),
            Type::F16 => Ok(IpldValue::Float(f64::from(d.f16()?))),
            Type::F32 => Ok(IpldValue::Float(f64::from(d.f32()?))),
            Type::F64 => Ok(IpldValue::Float(d.f64()?)),
            Type::Bytes => Ok(IpldValue::Bytes(d.bytes()?.to_vec())),
            Type::String => Ok(IpldValue::String(d.str()?.to_owned())),
            Type::Array => {
                let array_len = d.array()?.expect("array is definite");
                let mut array = Vec::with_capacity(array_len as usize);
                for _ in 0..array_len {
                    let obj = d.decode::<IpldValue>()?;
                    array.push(obj);
                }
                Ok(IpldValue::List(array))
            }
            Type::Map => {
                let map_len = d.map()?.expect("map is definite");
                let mut map = BTreeMap::new();
                for _ in 0..map_len {
                    let k = d.decode::<String>()?;
                    let v = d.decode::<IpldValue>()?;
                    map.insert(k, v);
                }
                Ok(IpldValue::Map(map))
            }
            Type::Tag => {
                let cid = d.decode::<Cid>()?;
                Ok(IpldValue::Link(cid))
            }
            Type::Break | Type::Unknown(_) | Type::Undefined | Type::Simple => {
                Err(decode::Error::Message("unexpected type"))
            }
        }
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of IPLD data model.

#![deny(missing_docs)]

mod error;
mod store;
#[macro_use]
mod value;

pub use self::error::{IpldError, Result};
pub use self::store::IpldStore;
pub use self::value::{Bytes, Integer, Map, MapKey, Value};

/// Convert JSON object into an IPLD value.
pub fn json_to_ipld<T>(value: &T) -> Result<Value, IpldError>
where
    T: ?Sized + serde::ser::Serialize,
{
    let json = serde_json::to_string(value).expect("`value` must be a JSON encoded object; qed");
    let value = serde_json::from_str::<Value>(&json)?;
    Ok(value)
}

/// Interpret a `Value` as an instance of type `T`.
pub fn json_from_ipld<T>(value: &Value) -> Result<T, IpldError>
where
    T: serde::de::DeserializeOwned,
{
    let json = serde_json::to_string(value).expect("`value` must be a JSON encoded object; qed");
    let value = serde_json::from_str::<T>(&json)?;
    Ok(value)
}

/// Convert CBOR object into an IPLD value.
pub fn cbor_to_ipld<T>(value: &T) -> Result<Value, IpldError>
where
    T: ?Sized + minicbor::encode::Encode,
{
    let cbor = minicbor::to_vec(value).expect("`value` must be a CBOR encoded object; qed");
    let value = minicbor::decode::<Value>(&cbor)?;
    Ok(value)
}

/// Interpret a `Value` as an instance of type `T`.
pub fn cbor_from_ipld<T>(value: &Value) -> Result<T, IpldError>
where
    T: for<'b> minicbor::decode::Decode<'b>,
{
    let cbor = minicbor::to_vec(value).expect("`value` must be a CBOR encoded object; qed");
    let value = minicbor::decode::<T>(&cbor)?;
    Ok(value)
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod block;
mod error;
#[macro_use]
mod value;

pub use self::block::IpldBlock;
pub use self::error::IpldError;
pub use self::value::{Bytes, Integer, IpldValue, Map, MapKey};

/// Convert JSON object into an IPLD value.
pub fn json_to_ipld<T>(value: &T) -> Result<IpldValue, IpldError>
where
    T: ?Sized + serde::ser::Serialize,
{
    let json = serde_json::to_string(value).unwrap();
    let value =
        serde_json::from_str::<IpldValue>(&json).map_err(|e| IpldError::Codec(e.to_string()))?;
    Ok(value)
}

/// Interpret a `IpldValue` as an instance of type `T`.
pub fn json_from_ipld<T>(value: &IpldValue) -> Result<T, IpldError>
where
    T: serde::de::DeserializeOwned,
{
    let json = serde_json::to_string(value).unwrap();
    let value = serde_json::from_str::<T>(&json).map_err(|e| IpldError::Codec(e.to_string()))?;
    Ok(value)
}

/// Convert CBOR object into an IPLD value.
pub fn cbor_to_ipld<T>(value: &T) -> Result<IpldValue, IpldError>
where
    T: ?Sized + minicbor::encode::Encode,
{
    let cbor = minicbor::to_vec(value).unwrap();
    let value =
        minicbor::decode::<IpldValue>(&cbor).map_err(|e| IpldError::Codec(e.to_string()))?;
    Ok(value)
}

/// Interpret a `IpldValue` as an instance of type `T`.
pub fn cbor_from_ipld<T>(value: &IpldValue) -> Result<T, IpldError>
where
    T: for<'b> minicbor::decode::Decode<'b>,
{
    let cbor = minicbor::to_vec(value).unwrap();
    let value = minicbor::decode::<T>(&cbor).map_err(|e| IpldError::Codec(e.to_string()))?;
    Ok(value)
}

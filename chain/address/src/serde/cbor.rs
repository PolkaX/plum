// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;

use serde::{de, ser};
use serde_bytes::{ByteBuf, Bytes, Deserialize, Serialize};

use crate::address::Address;
use crate::protocol::Protocol;

/// CBOR serialization
pub fn serialize<S>(address: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let bytes = address.as_bytes();
    let value = Bytes::new(&bytes);
    value.serialize(serializer)
}

/// CBOR deserialization
pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: de::Deserializer<'de>,
{
    let bytes = ByteBuf::deserialize(deserializer)?;
    let mut bytes = bytes.into_vec();
    let protocol = Protocol::try_from(bytes.remove(0)).map_err(de::Error::custom)?;
    Ok(Address::new(protocol, bytes).map_err(de::Error::custom)?)
}

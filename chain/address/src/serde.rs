use std::convert::TryFrom;
use std::str::FromStr;

use serde1::{de, ser};
use serde_bytes::ByteBuf;

use crate::address::Address;
use crate::protocol::Protocol;

impl ser::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let bytes = self.as_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes = ByteBuf::deserialize(deserializer)?;
        let mut bytes = bytes.into_vec();
        let protocol = Protocol::try_from(bytes.remove(0)).map_err(de::Error::custom)?;
        Ok(Self::new(protocol, bytes).map_err(de::Error::custom)?)
    }
}

/// A JSONify address.
pub struct JsonAddress(Address);

impl JsonAddress {
    /// Create a JSONify address from raw address.
    pub fn new(address: Address) -> Self {
        Self(address)
    }

    /// Unwrap the raw address.
    pub fn into_inner(self) -> Address {
        self.0
    }
}

impl ser::Serialize for JsonAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = serde_json::json!(self.0.to_string());
        value.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for JsonAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let addr = String::deserialize(deserializer)?;
        Ok(JsonAddress(
            Address::from_str(&addr).map_err(de::Error::custom)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::{set_network, Network};

    #[test]
    fn cbor_serde() {
        let id_addr = Address::new_id_addr(12_512_063u64).unwrap();
        let cbor = serde_cbor::to_vec(&id_addr).unwrap();
        assert_eq!(cbor, [69, 0, 191, 214, 251, 5]);
        assert_eq!(id_addr, serde_cbor::from_slice(&cbor).unwrap());
    }

    #[test]
    fn json_serde() {
        unsafe { set_network(Network::Test) };
        let id_addr = Address::new_id_addr(1024).unwrap();
        assert_eq!(id_addr.to_string(), "t01024");

        let json_id_addr = id_addr.clone().into_jsonify_address();
        let json = serde_json::to_string(&json_id_addr).unwrap();
        assert_eq!(json, "\"t01024\"");
        let json_id_addr: JsonAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(json_id_addr.into_inner(), id_addr);
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use crate::address::Address;

// Implement default serialization for Address.
impl ser::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

// Implement default deserialization for Address.
impl<'de> de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// Address CBOR serialization/deserialization
pub mod cbor {
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
        Bytes::new(&bytes).serialize(serializer)
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
}

/// Address JSON serialization/deserialization
pub mod json {
    use std::str::FromStr;

    use serde::{de, ser, Deserialize, Serialize};

    use crate::address::Address;

    /// JSON serialization
    pub fn serialize<S>(address: &Address, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        address.to_string().serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let addr = String::deserialize(deserializer)?;
        Ok(Address::from_str(&addr).map_err(de::Error::custom)?)
    }
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};

    use crate::{address_cbor, address_json, set_network, Address, Network};

    #[test]
    fn address_default_serde() {
        let id_addr = Address::new_id_addr(12_512_063u64).unwrap();
        let ser = serde_cbor::to_vec(&id_addr).unwrap();
        assert_eq!(ser, [69, 0, 191, 214, 251, 5]);
        let de = serde_cbor::from_slice::<Address>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CborAddress(#[serde(with = "address_cbor")] Address);

    #[test]
    fn address_cbor_serde() {
        let id_addr = CborAddress(Address::new_id_addr(12_512_063u64).unwrap());
        let ser = serde_cbor::to_vec(&id_addr).unwrap();
        assert_eq!(ser, [69, 0, 191, 214, 251, 5]);
        let de = serde_cbor::from_slice::<CborAddress>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct JsonAddress(#[serde(with = "address_json")] Address);

    #[test]
    fn address_json_serde() {
        unsafe { set_network(Network::Test) };
        let id_addr = JsonAddress(Address::new_id_addr(1024).unwrap());
        assert_eq!(id_addr.0.to_string(), "t01024");
        let ser = serde_json::to_string(&id_addr).unwrap();
        assert_eq!(ser, "\"t01024\"");
        let de = serde_json::from_str::<JsonAddress>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser};

use crate::address::Address;

// Implement CBOR serialization for Address.
impl encode::Encode for Address {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self.as_bytes())?.ok()
    }
}

// Implement CBOR deserialization for Address.
impl<'b> decode::Decode<'b> for Address {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        Ok(Address::new_from_bytes(bytes)
            .map_err(|_| decode::Error::Message("expected address bytes"))?)
    }
}

// Implement JSON serialization for Address.
impl ser::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

// Implement JSON deserialization for Address.
impl<'de> de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let addr = String::deserialize(deserializer)?;
        addr.parse::<Address>().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::{set_network, Address, Network};

    #[test]
    fn address_cbor_serde() {
        let id_addr = Address::new_id_addr(12_512_063u64).unwrap();
        let ser = minicbor::to_vec(&id_addr).unwrap();
        assert_eq!(ser, [69, 0, 191, 214, 251, 5]);
        let de = minicbor::decode::<Address>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }

    #[test]
    fn address_json_serde() {
        unsafe { set_network(Network::Test) };
        let id_addr = Address::new_id_addr(1024).unwrap();
        assert_eq!(id_addr.to_string(), "t01024");
        let ser = serde_json::to_string(&id_addr).unwrap();
        assert_eq!(ser, "\"t01024\"");
        let de = serde_json::from_str::<Address>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }
}

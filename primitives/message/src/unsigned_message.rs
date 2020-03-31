// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec};
use serde::{de, ser};

use plum_address::Address;
use plum_bigint::BigInt;

/// The unsigned message.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct UnsignedMessage {
    /// The receiver of the unsigned message.
    pub to: Address,
    /// The sender of the unsigned message.
    pub from: Address,
    /// The nonce.
    pub nonce: u64,
    /// The value.
    pub value: BigInt,

    /// The price of gas.
    pub gas_price: BigInt,
    /// The limit of gas.
    pub gas_limit: BigInt,

    /// The method.
    pub method: u64,
    /// The params of method.
    pub params: Vec<u8>,
}

impl UnsignedMessage {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        let data = serde_cbor::to_vec(self)
            .expect("CBOR serialization of UnsignedMessage shouldn't be failed");
        self.cid_with_data(data)
    }

    /// Convert to the CID with the given CBOR serialized data of UnsignedMessage.
    ///
    /// For cases where serialized data of the UnsignedMessage is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        let hash = multihash::Blake2b256::digest(data.as_ref());
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    /// Return the required funds.
    pub fn required_funds(&self) -> BigInt {
        self.value.clone() + (&self.gas_price * &self.gas_limit)
    }
}

impl ser::Serialize for UnsignedMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for UnsignedMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// UnsignedMessage CBOR serialization/deserialization.
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_address::{address_cbor, Address};
    use plum_bigint::{bigint_cbor, BigInt};

    use super::UnsignedMessage;

    #[derive(Serialize)]
    struct CborUnsignedMessageRef<'a>(
        #[serde(with = "address_cbor")] &'a Address,
        #[serde(with = "address_cbor")] &'a Address,
        &'a u64,
        #[serde(with = "bigint_cbor")] &'a BigInt,
        #[serde(with = "bigint_cbor")] &'a BigInt,
        #[serde(with = "bigint_cbor")] &'a BigInt,
        &'a u64,
        #[serde(with = "serde_bytes")] &'a Vec<u8>,
    );

    /// CBOR serialization.
    pub fn serialize<S>(unsigned_msg: &UnsignedMessage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborUnsignedMessageRef(
            &unsigned_msg.to,
            &unsigned_msg.from,
            &unsigned_msg.nonce,
            &unsigned_msg.value,
            &unsigned_msg.gas_price,
            &unsigned_msg.gas_limit,
            &unsigned_msg.method,
            &unsigned_msg.params,
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborUnsignedMessage(
        #[serde(with = "address_cbor")] Address,
        #[serde(with = "address_cbor")] Address,
        u64,
        #[serde(with = "bigint_cbor")] BigInt,
        #[serde(with = "bigint_cbor")] BigInt,
        #[serde(with = "bigint_cbor")] BigInt,
        u64,
        #[serde(with = "serde_bytes")] Vec<u8>,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<UnsignedMessage, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborUnsignedMessage(to, from, nonce, value, gas_price, gas_limit, method, params) =
            CborUnsignedMessage::deserialize(deserializer)?;
        Ok(UnsignedMessage {
            to,
            from,
            nonce,
            value,
            gas_price,
            gas_limit,
            method,
            params,
        })
    }
}

/// UnsignedMessage JSON serialization/deserialization.
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_address::{address_json, Address};
    use plum_bigint::{bigint_json, BigInt};

    use super::UnsignedMessage;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonUnsignedMessageRef<'a> {
        #[serde(with = "address_json")]
        to: &'a Address,
        #[serde(with = "address_json")]
        from: &'a Address,
        nonce: &'a u64,
        #[serde(with = "bigint_json")]
        value: &'a BigInt,
        #[serde(with = "bigint_json")]
        gas_price: &'a BigInt,
        #[serde(with = "bigint_json")]
        gas_limit: &'a BigInt,
        method: &'a u64,
        params: String,
    }

    /// JSON serialization.
    pub fn serialize<S>(unsigned_msg: &UnsignedMessage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonUnsignedMessageRef {
            to: &unsigned_msg.to,
            from: &unsigned_msg.from,
            nonce: &unsigned_msg.nonce,
            value: &unsigned_msg.value,
            gas_price: &unsigned_msg.gas_price,
            gas_limit: &unsigned_msg.gas_limit,
            method: &unsigned_msg.method,
            params: base64::encode(&unsigned_msg.params),
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonUnsignedMessage {
        #[serde(with = "address_json")]
        to: Address,
        #[serde(with = "address_json")]
        from: Address,
        nonce: u64,
        #[serde(with = "bigint_json")]
        value: BigInt,
        #[serde(with = "bigint_json")]
        gas_price: BigInt,
        #[serde(with = "bigint_json")]
        gas_limit: BigInt,
        method: u64,
        params: String,
    }

    /// JSON deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<UnsignedMessage, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonUnsignedMessage {
            to,
            from,
            nonce,
            value,
            gas_price,
            gas_limit,
            method,
            params,
        } = JsonUnsignedMessage::deserialize(deserializer)?;
        Ok(UnsignedMessage {
            to,
            from,
            nonce,
            value,
            gas_price,
            gas_limit,
            method,
            params: base64::decode(params).expect("base64 decode shouldn't be fail"),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use plum_address::{set_network, Address, Network};

    use super::UnsignedMessage;

    fn new_unsigned_message() -> UnsignedMessage {
        let to_pubkey = [
            82, 253, 252, 7, 33, 130, 101, 79, 22, 63, 95, 15, 154, 98, 29, 114, 149, 102, 199, 77,
            16, 3, 124, 77, 123, 187, 4, 7, 209, 226, 198, 73, 129, 133, 90, 216, 104, 29, 13, 134,
            209, 233, 30, 0, 22, 121, 57, 203,
        ];
        let from_pubkey = [
            47, 130, 130, 203, 226, 249, 105, 111, 49, 68, 192, 170, 76, 237, 86, 219, 217, 103,
            220, 40, 151, 128, 106, 243, 190, 216, 166, 58, 202, 22, 225, 139, 104, 107, 160, 220,
            32, 140, 254, 206, 101, 189, 112, 162, 61, 160, 2, 107,
        ];

        UnsignedMessage {
            to: Address::new_bls_addr(&to_pubkey).unwrap(),
            from: Address::new_bls_addr(&from_pubkey).unwrap(),
            nonce: 197u64,
            value: Default::default(),
            gas_limit: 126_723u64.into(),
            gas_price: 1_776_234u64.into(),
            method: 1_231_254u64,
            params: b"some bytes, idk. probably at least ten of them".to_vec(),
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CborUnsignedMessage(#[serde(with = "super::cbor")] UnsignedMessage);

    #[test]
    fn unsigned_message_cbor_serde() {
        unsafe {
            set_network(Network::Test);
        }
        let unsigned_message = CborUnsignedMessage(new_unsigned_message());
        let expected = vec![
            136, 88, 49, 3, 82, 253, 252, 7, 33, 130, 101, 79, 22, 63, 95, 15, 154, 98, 29, 114,
            149, 102, 199, 77, 16, 3, 124, 77, 123, 187, 4, 7, 209, 226, 198, 73, 129, 133, 90,
            216, 104, 29, 13, 134, 209, 233, 30, 0, 22, 121, 57, 203, 88, 49, 3, 47, 130, 130, 203,
            226, 249, 105, 111, 49, 68, 192, 170, 76, 237, 86, 219, 217, 103, 220, 40, 151, 128,
            106, 243, 190, 216, 166, 58, 202, 22, 225, 139, 104, 107, 160, 220, 32, 140, 254, 206,
            101, 189, 112, 162, 61, 160, 2, 107, 24, 197, 64, 68, 0, 27, 26, 106, 68, 0, 1, 239, 3,
            26, 0, 18, 201, 150, 88, 46, 115, 111, 109, 101, 32, 98, 121, 116, 101, 115, 44, 32,
            105, 100, 107, 46, 32, 112, 114, 111, 98, 97, 98, 108, 121, 32, 97, 116, 32, 108, 101,
            97, 115, 116, 32, 116, 101, 110, 32, 111, 102, 32, 116, 104, 101, 109,
        ];

        let ser = serde_cbor::to_vec(&unsigned_message).unwrap();
        assert_eq!(ser, expected);
        let de = serde_cbor::from_slice::<CborUnsignedMessage>(&ser).unwrap();
        assert_eq!(de, unsigned_message);
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct JsonUnsignedMessage(#[serde(with = "super::json")] UnsignedMessage);

    #[test]
    fn unsigned_message_json_serde() {
        unsafe {
            set_network(Network::Test);
        }
        let unsigned_message = JsonUnsignedMessage(new_unsigned_message());
        let expected = "{\
            \"To\":\"t3kl67ybzbqjsu6fr7l4hzuyq5okkwnr2ncabxytl3xmcapupcyzeydbk23bub2dmg2hur4aawpe44w3wptsvq\",\
            \"From\":\"t3f6bifs7c7fuw6mkeycvez3kw3pmwpxbis6agv4563ctdvsqw4gfwq25a3qqiz7womw6xbir5uabgwykazd5a\",\
            \"Nonce\":197,\
            \"Value\":\"0\",\
            \"GasPrice\":\"1776234\",\
            \"GasLimit\":\"126723\",\
            \"Method\":1231254,\
            \"Params\":\"c29tZSBieXRlcywgaWRrLiBwcm9iYWJseSBhdCBsZWFzdCB0ZW4gb2YgdGhlbQ==\"\
        }";

        let ser = serde_json::to_string(&unsigned_message).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<JsonUnsignedMessage>(&ser).unwrap();
        assert_eq!(de, unsigned_message);
    }
}

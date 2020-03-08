// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use plum_address::Address;
use plum_bigint::BigInt;

/// The unsigned message.
#[derive(Eq, PartialEq, Clone, Debug)]
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
    struct TupleUnsignedMessage<'a>(
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
        let tuple_unsigned_msg = TupleUnsignedMessage(
            &unsigned_msg.to,
            &unsigned_msg.from,
            &unsigned_msg.nonce,
            &unsigned_msg.value,
            &unsigned_msg.gas_price,
            &unsigned_msg.gas_limit,
            &unsigned_msg.method,
            &unsigned_msg.params,
        );
        tuple_unsigned_msg.serialize(serializer)
    }

    #[derive(Deserialize)]
    struct OwnedTupleUnsignedMessage(
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
        let OwnedTupleUnsignedMessage(to, from, nonce, value, gas_price, gas_limit, method, params) =
            OwnedTupleUnsignedMessage::deserialize(deserializer)?;
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

#[cfg(test)]
mod tests {
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
            gas_limit: 126723u64.into(),
            gas_price: 1776234u64.into(),
            method: 1231254u64,
            params: b"some bytes, idk. probably at least ten of them".to_vec(),
        }
    }

    #[test]
    fn unsigned_message_cbor_serde() {
        unsafe {
            set_network(Network::Test);
        }
        let unsigned_message = new_unsigned_message();
        let expected = [
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

        let cbor = serde_cbor::to_vec(&unsigned_message).unwrap();
        assert_eq!(cbor, &expected[..]);
        let out: UnsignedMessage = serde_cbor::from_slice(&cbor).unwrap();
        assert_eq!(out, unsigned_message);
    }
}

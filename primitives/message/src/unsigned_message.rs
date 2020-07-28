// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::{anyhow, Result};
use cid::{Cid, Codec, IntoExt};
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_address::Address;
use plum_bigint::{bigint_json, BigInt, BigIntRefWrapper, BigIntWrapper};
use plum_types::{Gas, MethodNum};

/// The unsigned message.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnsignedMessage {
    ///
    pub version: i64,

    /// The receiver of the unsigned message.
    pub to: Address,
    /// The sender of the unsigned message.
    pub from: Address,
    /// The nonce.
    pub nonce: u64,
    /// The value.
    #[serde(with = "bigint_json")]
    pub value: BigInt,

    /// The price of gas.
    #[serde(with = "bigint_json")]
    pub gas_price: BigInt,
    /// The limit of gas.
    #[serde(with = "bigint_json")]
    pub gas_limit: BigInt,

    /// The method.
    pub method: MethodNum,
    /// The params of method.
    #[serde(with = "plum_bytes::base64")]
    pub params: Vec<u8>,
}

impl UnsignedMessage {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        let data = minicbor::to_vec(self)
            .expect("CBOR serialization of UnsignedMessage shouldn't be failed");
        self.cid_with_data(data)
    }

    /// Convert to the CID with the given CBOR serialized data of UnsignedMessage.
    ///
    /// For cases where serialized data of the UnsignedMessage is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        let hash = multihash::Blake2b256::digest(data.as_ref()).into_ext();
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    /// Return the required funds.
    pub fn required_funds(&self) -> BigInt {
        self.value.clone() + (&self.gas_price * &self.gas_limit)
    }

    /// Returns true if this message is valid to be include in a block.
    pub fn validate_for_block_inclusion(&self, min_gas: Gas) -> Result<()> {
        if self.version != 0 {
            return Err(anyhow!("version unsupported"));
        }

        if self.value < 0.into() {
            return Err(anyhow!("value field cannot be negative"));
        }

        if self.value > *plum_types::TOTAL_FILECOIN {
            return Err(anyhow!(
                "value field cannot be greater than total filecoin supply"
            ));
        }

        if self.gas_price < 0.into() {
            return Err(anyhow!("gas_price field cannot be negative"));
        }

        if self.gas_limit > plum_types::BLOCK_GAS_LIMIT.into() {
            return Err(anyhow!(
                "gas_limit field cannot be greater than a block's gas limit"
            ));
        }

        // since prices might vary with time, this is technically semantic validation
        if self.gas_limit < min_gas {
            return Err(anyhow!(
                "gas_limit field cannot be less than the cost of storing a message on chain",
            ));
        }

        Ok(())
    }
}

// Implement CBOR serialization for UnsignedMessage.
impl encode::Encode for UnsignedMessage {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(9)?
            .i64(self.version)?
            .encode(&self.to)?
            .encode(&self.from)?
            .u64(self.nonce)?
            .encode(BigIntRefWrapper::from(&self.value))?
            .encode(BigIntRefWrapper::from(&self.gas_price))?
            .encode(BigIntRefWrapper::from(&self.gas_limit))?
            .u64(self.method)?
            .bytes(&self.params)?
            .ok()
    }
}

// Implement CBOR deserialization for UnsignedMessage.
impl<'b> decode::Decode<'b> for UnsignedMessage {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(9));
        Ok(UnsignedMessage {
            version: d.i64()?,
            to: d.decode::<Address>()?,
            from: d.decode::<Address>()?,
            nonce: d.u64()?,
            value: d.decode::<BigIntWrapper>()?.into_inner(),
            gas_price: d.decode::<BigIntWrapper>()?.into_inner(),
            gas_limit: d.decode::<BigIntWrapper>()?.into_inner(),
            method: d.u64()?,
            params: d.bytes()?.to_vec(),
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
            version: 0,
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

    #[test]
    fn unsigned_message_cbor_serde() {
        unsafe {
            set_network(Network::Test);
        }
        let unsigned_message = new_unsigned_message();
        let expected = vec![
            137, 0, 88, 49, 3, 82, 253, 252, 7, 33, 130, 101, 79, 22, 63, 95, 15, 154, 98, 29, 114,
            149, 102, 199, 77, 16, 3, 124, 77, 123, 187, 4, 7, 209, 226, 198, 73, 129, 133, 90,
            216, 104, 29, 13, 134, 209, 233, 30, 0, 22, 121, 57, 203, 88, 49, 3, 47, 130, 130, 203,
            226, 249, 105, 111, 49, 68, 192, 170, 76, 237, 86, 219, 217, 103, 220, 40, 151, 128,
            106, 243, 190, 216, 166, 58, 202, 22, 225, 139, 104, 107, 160, 220, 32, 140, 254, 206,
            101, 189, 112, 162, 61, 160, 2, 107, 24, 197, 64, 68, 0, 27, 26, 106, 68, 0, 1, 239, 3,
            26, 0, 18, 201, 150, 88, 46, 115, 111, 109, 101, 32, 98, 121, 116, 101, 115, 44, 32,
            105, 100, 107, 46, 32, 112, 114, 111, 98, 97, 98, 108, 121, 32, 97, 116, 32, 108, 101,
            97, 115, 116, 32, 116, 101, 110, 32, 111, 102, 32, 116, 104, 101, 109,
        ];

        let ser = minicbor::to_vec(&unsigned_message).unwrap();
        assert_eq!(ser, expected);
        let de = minicbor::decode::<UnsignedMessage>(&ser).unwrap();
        assert_eq!(de, unsigned_message);
    }

    #[test]
    fn unsigned_message_json_serde() {
        unsafe {
            set_network(Network::Test);
        }
        let unsigned_message = new_unsigned_message();
        let expected = "{\
            \"Version\":0,\
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
        let de = serde_json::from_str::<UnsignedMessage>(&ser).unwrap();
        assert_eq!(de, unsigned_message);
    }
}

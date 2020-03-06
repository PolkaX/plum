// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use address::Address;
use block_format::BasicBlock;
use cid::Cid;
use core::convert::TryInto;
use plum_bigint::BigInt;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::ops::Add;
use std::ops::Mul;

use crate::{into_cid, to_storage_block, StorageBlockError};

#[derive(Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Message {
    pub to: Address,
    pub from: Address,
    pub nonce: u64,
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub value: BigInt,
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub gas_price: BigInt,
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub gas_limit: BigInt,
    pub method: u64,
    #[serde(with = "serde_bytes")]
    pub params: Vec<u8>,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.clone().cid() == other.clone().cid()
    }
}

impl Eq for Message {}

impl TryInto<BasicBlock> for Message {
    type Error = StorageBlockError;
    fn try_into(self) -> std::result::Result<BasicBlock, Self::Error> {
        to_storage_block(&self)
    }
}

impl Message {
    pub fn cid(self) -> Cid {
        into_cid(self)
    }

    pub fn required_funds(&self) -> BigInt {
        self.value.clone().add(&self.gas_price).mul(&self.gas_limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use address::Address;
    use cid::AsCidRef;

    fn new_message() -> Message {
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

        Message {
            to: Address::new_bls_addr(&to_pubkey).unwrap(),
            from: Address::new_bls_addr(&from_pubkey).unwrap(),
            nonce: 197u64,
            method: 1231254u64,
            params: b"some bytes, idk. probably at least ten of them".to_vec(),
            gas_limit: 126723u64.into(),
            gas_price: 1776234u64.into(),
            value: Default::default(),
        }
    }

    #[test]
    fn message_serde_should_work() {
        let message = new_message();
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

        let ser = serde_cbor::to_vec(&message).unwrap();
        assert_eq!(ser, &expected[..]);
        let de: Message = serde_cbor::from_slice(&ser).unwrap();
        assert_eq!(message, de);
    }

    #[test]
    fn message_to_storge_block_should_work() {
        let message = new_message();
        let storage_block: BasicBlock = message.try_into().unwrap();
        assert_eq!(
            "bafy2bzacedanpgfis26fj5tthvjc6giupcgiwaipjrlxre3fnyfsya2437k4k",
            storage_block.cid().to_string()
        );
    }
}

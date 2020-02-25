// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use rust_ipld_cbor::bigint::CborBigInt;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Eq, PartialEq, Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct MessageReceipt {
    pub exit_code: u8,
    #[serde(with = "serde_bytes")]
    pub ret: Vec<u8>,
    pub gas_used: CborBigInt,
}

#[test]
fn message_receipt_serde_should_work() {
    let receipt = MessageReceipt {
        exit_code: 127u8,
        ret: b"ret".to_vec(),
        gas_used: CborBigInt(1776234.into()),
    };
    let expected = [131, 24, 127, 67, 114, 101, 116, 68, 0, 27, 26, 106];

    let ser = serde_cbor::to_vec(&receipt).unwrap();
    assert_eq!(ser, &expected[..]);
    let de: MessageReceipt = serde_cbor::from_slice(&ser).unwrap();
    assert_eq!(receipt, de);
}

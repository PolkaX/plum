// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::Address;

use crate::bigint::BigInt;

pub struct Message {
    pub to: Address,
    pub from: Address,
    pub nonce: u64,
    pub value: BigInt,
    pub gas_price: BigInt,
    pub gas_limit: BigInt,
    pub method: u64,
    pub params: Vec<u8>,
}

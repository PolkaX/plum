// Copyright 2019 PolkaX. Licensed under GPL-3.0.

use crate::types::{Address, BigInt};

pub struct Message {
    to: Address,
    from: Address,
    nonce: u64,
    value: BigInt,
    gas_price: BigInt,
    gas_limit: BigInt,
    method: u64,
    params: Vec<u8>,
}

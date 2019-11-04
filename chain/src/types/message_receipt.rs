// Copyright 2019 chainnet.tech

use crate::types::BigInt;

pub struct MessageReceipt {
    exit_code: u8,
    ret: Vec<u8>,
    gas_used: BigInt,
}

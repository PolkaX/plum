// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::BigInt;

pub struct MessageReceipt {
    exit_code: u8,
    ret: Vec<u8>,
    gas_used: BigInt,
}

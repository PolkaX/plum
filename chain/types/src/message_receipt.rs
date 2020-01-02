// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::bigint::BigInt;

pub struct MessageReceipt {
    pub exit_code: u8,
    pub ret: Vec<u8>,
    pub gas_used: BigInt,
}

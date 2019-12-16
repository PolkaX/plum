// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::{BigInt, Cid};

pub struct Actor {
    code: Cid,
    head: Cid,
    nonce: u64,
    balance: BigInt,
}

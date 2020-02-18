// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{bigint::BigInt, Cid};

pub struct Actor {
    pub code: Cid,
    pub head: Cid,
    pub nonce: u64,
    pub balance: BigInt,
}

// Copyright 2019 chainnet.tech

use crate::types::{Cid, BigInt};
pub struct Actor {
    code: Cid,
    head: Cid,
    nonce: u64,
    balance: BigInt,
}

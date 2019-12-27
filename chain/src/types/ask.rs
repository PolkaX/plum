// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::{BigInt, Signature};
use address::Address;

pub struct StorageAsk {
    price: BigInt,
    min_piece_size: u64,
    miner: Address,
    timestamp: u64,
    expiry: u64,
    seq_no: u64,
}

pub struct SignedStorageAsk {
    ask: StorageAsk,
    signature: Signature,
}

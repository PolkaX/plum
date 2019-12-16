// Copyright 2019 PolkaX. Licensed under GPL-3.0.

use crate::types::{Address, BigInt, Signature};

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

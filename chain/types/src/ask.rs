// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::Address;

use crate::{bigint::BigInt, signature::Signature};

pub struct StorageAsk {
    pub price: BigInt,
    pub min_piece_size: u64,
    pub miner: Address,
    pub timestamp: u64,
    pub expiry: u64,
    pub seq_no: u64,
}

pub struct SignedStorageAsk {
    pub ask: StorageAsk,
    pub signature: Signature,
}

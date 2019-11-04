// Copright 2019 chainnet.tech

use crate::{BigInt, Address, Signature};

pub struct StorageAsk {
    price: BigInt,
    min_piece_size: u64,
    miner: Address,
    timestamp: u64,
    expiry: u64,
    seqno: u64,
}

pub struct SignedStorageAsk {
    ask: StorageAsk,
    Signature: Signature,
}

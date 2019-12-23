// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::{BigInt, Cid, Signature};
use address::Address;

pub struct Ticket {
    vrf_proof: Vec<u8>,
}

pub type ElectionProof = Vec<u8>;

pub struct BlockHeader {
    miner: Address,
    tickets: Vec<Ticket>,
    election_proof: Vec<u8>,
    parents: Vec<Cid>,
    parent_weight: BigInt,
    height: u64,
    parent_state_root: Cid,
    parent_message_receipts: Cid,
    messages: Cid,
    bls_aggregate: Signature,
    timestamp: u64,
    block_sig: Signature,
}

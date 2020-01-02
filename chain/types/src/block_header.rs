// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::Address;

use crate::{bigint::BigInt, signature::Signature, Cid};

pub struct Ticket {
    pub vrf_proof: Vec<u8>,
}

pub type ElectionProof = Vec<u8>;

pub struct BlockHeader {
    pub miner: Address,
    pub tickets: Vec<Ticket>,
    pub election_proof: Vec<u8>,
    pub parents: Vec<Cid>,
    pub parent_weight: BigInt,
    pub height: u64,
    pub parent_state_root: Cid,
    pub parent_message_receipts: Cid,
    pub messages: Cid,
    pub bls_aggregate: Signature,
    pub timestamp: u64,
    pub block_sig: Signature,
}

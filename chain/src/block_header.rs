// Copyright 2019 chainnet.tech

use crate::{BigInt, Address, Cid, Signature};

pub struct Ticket {
    vrf_proof: Vec<u8>,
}

pub type ElectionProof = Vec<u8>;

pub struct BlockHeader {
    miner: Address,
    tickets: Vec<Ticket>,
    election_proof: Vec<u8>,
    parents: Vec<Cid>,
    parentWeight: BigInt,
    Height: u64,
    parent_state_root: Cid,
    parent_message_receipts: Cid,
    messages: Cid,
    bls_aggregate: Signature,
    timestamp: u64,
    block_sig: Signature,
}

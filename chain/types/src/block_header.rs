// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::Address;

use crate::{bigint::BigInt, signature::Signature};

use block_format::BasicBlock;
use bytes::Bytes;
use cid::{AsCidRef, Cid, Codec, Hash, Prefix};
use core::convert::TryInto;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub vrf_proof: Vec<u8>,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EPostTicket {
    pub partial: Vec<u8>,
    pub sector_id: u64,
    pub challenge_index: u64,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EPostProof {
    pub proof: Vec<u8>,
    pub post_rand: Vec<u8>,
    pub candidates: Vec<EPostTicket>,
}

pub type ElectionProof = Vec<u8>;

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub miner: Address,
    pub ticket: Ticket,
    pub epost_proof: EPostProof,
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

impl TryInto<BasicBlock> for BlockHeader {
    type Error = anyhow::Error;
    fn try_into(self) -> std::result::Result<BasicBlock, Self::Error> {
        let data = Bytes::from(serde_cbor::to_vec(&self)?);

        let prefix = Prefix::new_prefix_v1(Codec::DagCBOR, Hash::Blake2b256);
        let cid = prefix.sum(&data)?;
        let block = BasicBlock::new_with_cid(data, cid)?;

        Ok(block)
    }
}

impl BlockHeader {
    pub fn cid(self) -> Cid {
        let blk: BasicBlock = self.try_into().expect("TODO: Check this later");
        blk.cid().clone()
    }
}

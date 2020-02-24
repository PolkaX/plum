// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{into_cid, to_storage_block, StorageBlockError};
use address::Address;
use block_format::BasicBlock;
use cid::Cid;
use core::convert::TryInto;
use rust_ipld_cbor::bigint::CborBigInt;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Message {
    pub to: Address,
    pub from: Address,
    pub nonce: u64,
    pub value: CborBigInt,
    pub gas_price: CborBigInt,
    pub gas_limit: CborBigInt,
    pub method: u64,
    pub params: Vec<u8>,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.clone().cid() == other.clone().cid()
    }
}

impl Eq for Message {}

impl TryInto<BasicBlock> for Message {
    type Error = StorageBlockError;
    fn try_into(self) -> std::result::Result<BasicBlock, Self::Error> {
        to_storage_block(&self)
    }
}

impl Message {
    pub fn cid(self) -> Cid {
        into_cid(self)
    }
}

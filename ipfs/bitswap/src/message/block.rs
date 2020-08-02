// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use ipfs_block::IpfsBlock;

use super::proto;
use crate::message::prefix::Prefix;

/// The block type in the bitswap protocol.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    prefix: Prefix,
    data: Vec<u8>,
}

impl From<&IpfsBlock> for Block {
    fn from(block: &IpfsBlock) -> Self {
        Self {
            prefix: block.cid().into(),
            data: block.data().to_vec(),
        }
    }
}

impl From<Block> for proto::message::Block {
    fn from(block: Block) -> Self {
        Self {
            prefix: block.prefix.to_bytes(),
            data: block.data,
        }
    }
}

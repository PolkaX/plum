// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipfs_blockstore::BlockStore;
use ipld::IpldValue;

///
#[derive(Clone, PartialEq, Debug)]
pub enum Link {
    ///
    Cid(Cid),
    ///
    Cache(Box<Node>),
}

/// Each node in an IPLD vector stores the width, the height of the node,
/// starting from 0 where values are stored,
/// and a data array to contain values (for height 0), or child node CIDs (for heights above 1).
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    ///
    Links(Vec<Link>),
    ///
    Leaves(Vec<IpldValue>),
}

impl Default for Node {
    fn default() -> Self {
        Node::Leaves(vec![])
    }
}

// Implement CBOR serialization for Node.
impl encode::Encode for Node {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        todo!()
    }
}

// Implement CBOR deserialization for Node.
impl<'b> decode::Decode<'b> for Node {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        todo!()
    }
}

impl Node {
    ///
    pub fn get<BS: BlockStore>(
        &self,
        store: &BS,
        height: u64,
        index: usize,
    ) -> Result<Option<IpldValue>, String> {
        todo!()
    }

    ///
    pub fn set<BS: BlockStore>(
        &mut self,
        store: &BS,
        height: u64,
        index: usize,
        value: IpldValue,
    ) -> Result<bool, String> {
        todo!()
    }

    pub fn delete<BS: BlockStore>(
        &mut self,
        store: &BS,
        height: u64,
        index: usize,
    ) -> Result<Option<IpldValue>, String> {
        todo!()
    }

    ///
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    ///
    pub fn flush<BS>(&mut self, store: &BS, height: u64) -> Result<(), String> {
        todo!()
    }
}

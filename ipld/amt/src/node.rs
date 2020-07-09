// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipld::IpldValue;

/// The only configurable parameter of an IPLD Vector.
/// This parameter must be consistent across all nodes in a Vector.
///
/// Mutations cannot involve changes in width or
/// joining multiple parts of a Vector with differing width values.
///
/// `WIDTH` must be an integer, of at least 2.
pub const WIDTH: usize = 8;

///
pub const MAX_INDEX: u64 = 1 << 48; // fairly arbitrary, but I don't want to overflow/underflow in nodesForHeight

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
    Values(Vec<IpldValue>),
}

impl Default for Node {
    fn default() -> Self {
        Node::Values(vec![])
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

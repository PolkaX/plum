// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipld::IpldValue;

use crate::node::Node;

///
pub struct KeyValuePair(Vec<u8>, IpldValue);

///
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Pointer {
    KVs(Vec<KeyValuePair>),
    Link(Cid),
    // cached node to avoid too many serialization operations
    Cache(Box<Node>),
}

impl Default for Pointer {
    fn default() -> Self {
        Self::KVs(vec![])
    }
}

// Implement CBOR serialization for Pointer.
impl encode::Encode for Pointer {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        todo!()
    }
}

// Implement CBOR deserialization for Pointer.
impl<'b> decode::Decode<'b> for Pointer {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        todo!()
    }
}

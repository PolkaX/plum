// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};

use crate::bitfield::U256;
use crate::pointer::Pointer;

///
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Node {
    bitfield: U256,
    pointers: Vec<Pointer>,
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

impl Node {}

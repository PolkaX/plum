// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};

use crate::node::Node;

/// Root of Amt nodes, which stores current tree height and count as well.
#[derive(Clone, PartialEq, Debug)]
pub struct Root {
    pub(crate) height: u64,
    pub(crate) count: u64,
    pub(crate) node: Node,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            height: 0,
            count: 0,
            node: Node::default(),
        }
    }
}

// Implement CBOR serialization for Root.
impl encode::Encode for Root {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .u64(self.height)?
            .u64(self.count)?
            .encode(&self.node)?
            .ok()
    }
}

// Implement CBOR deserialization for Root.
impl<'b> decode::Decode<'b> for Root {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(Root {
            height: d.u64()?,
            count: d.u64()?,
            node: d.decode()?,
        })
    }
}

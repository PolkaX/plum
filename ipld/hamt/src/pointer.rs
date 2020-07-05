// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipfs_blockstore::BlockStore;
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
        match self {
            Pointer::Link(link) => e.map(1)?.str("0")?.encode(link)?.ok(),
            Pointer::KVs(kvs) => e.map(1)?.str("1")?.encode(kvs)?.ok(),
            Pointer::Cache(_) => panic!("Cannot serialize cached values"),
        }
    }
}

// Implement CBOR deserialization for Pointer.
impl<'b> decode::Decode<'b> for Pointer {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let map_len = d.map()?;
        assert_eq!(map_len, Some(1));
        match d.str()? {
            "0" => Ok(Pointer::Link(d.decode()?)),
            "1" => Ok(Pointer::KVs(d.decode()?)),
            _ => Err(decode::Error::Message(
                "invalid pointer map key in cbor input",
            )),
        }
    }
}

impl Pointer {
    ///
    pub fn from_kvs<T: Into<Vec<KeyValuePair>>>(kvs: T) -> Self {
        Pointer::KVs(kvs.into())
    }

    ///
    pub fn from_link(link: Cid) -> Self {
        Pointer::Link(link)
    }
}

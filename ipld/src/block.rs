// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec, IntoExt};

/// A IPLD Block is a CID and the binary data value for that CID.
///
/// +-----+--------------------------------+
/// | CID | Data                           |
/// +-----+--------------------------------+
///
/// See [Concept: Block](https://github.com/ipld/specs/blob/master/block-layer/block.md) for details.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IpldBlock {
    cid: Cid,
    data: Vec<u8>,
}

impl IpldBlock {
    /// Create IPLD block from supported entity
    pub fn new<T: minicbor::Encode>(entity: T) -> Self {
        let data = minicbor::to_vec(&entity).unwrap();
        let hash = multihash::Blake2b256::digest(&data);
        let cid = Cid::new_v1(Codec::DagCBOR, hash.into_ext());
        Self { cid, data }
    }

    /// Return the Cid of the IPLD block.
    pub fn cid(&self) -> &Cid {
        &self.cid
    }

    /// Return the binary data value of the IPLD block.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl std::fmt::Display for IpldBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Block {}]", self.cid)
    }
}

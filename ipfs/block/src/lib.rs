// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of IPFS(IPLD) block.

#![deny(missing_docs)]

use std::fmt;

use cid::{Cid, Codec};

/// A IPFS(IPLD) Block is a CID and the binary data value for that CID.
///
/// +-----+--------------------------------+
/// | CID | Data                           |
/// +-----+--------------------------------+
///
/// See [Concept: Block](https://github.com/ipld/specs/blob/master/block-layer/block.md) for details.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Block {
    cid: Cid,
    data: Vec<u8>,
}

impl Block {
    /// Create IPFS(IPLD) block from supported entity
    pub fn new<T: minicbor::Encode>(entity: T) -> Self {
        let data = minicbor::to_vec(&entity).expect("`entity` must be a CBOR encoded object; qed");
        let hash = multihash::Blake2b256::digest(&data);
        let cid = Cid::new_v1(Codec::DagCBOR, hash);
        Self { cid, data }
    }

    /// Create IPFS(IPLD) block when the hash of the data is already known.
    ///
    /// # Safety
    ///
    /// It used to save time in situations where we are able to be confident
    /// that data and cid is correct.
    ///
    pub unsafe fn new_unchecked<T: Into<Vec<u8>>>(data: T, cid: Cid) -> Self {
        Self {
            cid,
            data: data.into(),
        }
    }

    /// Return the Cid of the IPFS(IPLD) block.
    pub fn cid(&self) -> &Cid {
        &self.cid
    }

    /// Return the binary data value of the IPFS(IPLD) block.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Block {}]", self.cid)
    }
}

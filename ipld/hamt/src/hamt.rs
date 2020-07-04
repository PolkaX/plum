// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use ipfs_blockstore::BlockStore;

use crate::node::Node;

/// Implementation of the HAMT data structure for IPLD.
pub struct Hamt<BS: BlockStore> {
    root: Node,
    bit_width: u32,
    store: BS,
}

impl<BS: BlockStore> PartialEq for Hamt<BS> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

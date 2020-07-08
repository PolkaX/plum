// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use ipfs_blockstore::BlockStore;

use crate::root::Root;

///
#[derive(Debug)]
pub struct Amt<BS: BlockStore> {
    root: Root,
    store: BS,
}

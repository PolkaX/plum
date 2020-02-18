// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{block_header::BlockHeader, Cid};

pub struct TipSet {
    pub cids: Vec<Cid>,
    pub blks: Vec<BlockHeader>,
    pub height: u64,
}

// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::{BlockHeader, Cid};

pub struct TipSet {
    cids: Vec<Cid>,
    blks: Vec<BlockHeader>,
    height: u64,
}

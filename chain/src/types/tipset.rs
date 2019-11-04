// Copyright 2019 chainnet.tech

use crate::types::{BlockHeader, Cid};

pub struct TipSet {
    cids: Vec<Cid>,
    blks: Vec<BlockHeader>,
    height: u64,
}

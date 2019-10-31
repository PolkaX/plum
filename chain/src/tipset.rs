// Copyright 2019 chainnet.tech

use crate::{BlockHeader, Cid};

pub struct TipSet {
    cids: Vec<Cid>,
    blks: Vec<BlockHeader>,
    height: u64,
}

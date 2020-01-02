// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::{block_header::BlockHeader, Cid};

pub struct BlockMsg {
    pub header: BlockHeader,
    pub bls_messages: Vec<Cid>,
    pub secp256k1_messages: Vec<Cid>,
}

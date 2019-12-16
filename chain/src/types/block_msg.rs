// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::{BlockHeader, Cid};

pub struct BlockMsg {
    header: BlockHeader,
    bls_messages: Vec<Cid>,
    secpk_messages: Vec<Cid>,
}

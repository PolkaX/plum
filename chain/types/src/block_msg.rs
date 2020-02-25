// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{block_header::BlockHeader, Cid};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Eq, PartialEq, Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct BlockMsg {
    pub header: BlockHeader,
    pub bls_messages: Vec<Cid>,
    pub secpk_messages: Vec<Cid>,
}

impl BlockMsg {
    pub fn cid(&self) -> Cid {
        self.header.clone().cid()
    }
}

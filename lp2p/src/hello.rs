// use cid::Cid;
use serde::{Deserialize, Serialize};

type Cid = u8;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub heaviest_tip_set: Cid,
    pub heaviest_tip_set_weight: u128,
    pub genesis_hash: Cid,
}

impl Message {
    pub fn new(heaviest_tip_set: Cid, heaviest_tip_set_weight: u128, genesis_hash: Cid) -> Self {
        Self {
            heaviest_tip_set,
            heaviest_tip_set_weight,
            genesis_hash,
        }
    }
}

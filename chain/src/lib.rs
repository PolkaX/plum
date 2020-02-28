// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

pub struct Info {
    pub heaviest_tip_set: Cid,
    pub heaviest_tip_set_weight: u128,
    pub genesis_hash: Cid,
    pub best_hash: Cid,
}

pub trait Client {
    fn info(&self) -> Info;
}

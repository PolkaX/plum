// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

pub mod types;

pub type Cid = u8;

pub struct Info {
    pub heaviest_tip_set: Cid,
    pub heaviest_tip_set_weight: u128,
    pub genesis_hash: Cid,
    pub best_hash: Cid,
}

pub trait Client {
    fn info(&self) -> Info;
}

#[cfg(test)]
mod tests {}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryInto;
use std::num::TryFromIntError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd)]
/// An epoch represents a single valid state in the blockchain
pub struct ChainEpoch(u64);

impl From<u64> for ChainEpoch {
    fn from(num: u64) -> ChainEpoch {
        ChainEpoch(num)
    }
}

impl From<ChainEpoch> for u64 {
    fn from(ce: ChainEpoch) -> u64 {
        ce.0
    }
}

impl ChainEpoch {
    /// Returns ChainEpoch based on the given unix timestamp
    pub fn new(timestamp: i64) -> Result<ChainEpoch, TryFromIntError> {
        Ok(ChainEpoch(timestamp.try_into()?))
    }
    // Returns chain epoch
    pub fn chain_epoch(&self) -> &u64 {
        &self.0
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.
use serde::{Deserialize, Serialize};

use std::borrow::{Borrow, BorrowMut};
use std::convert::TryInto;
use std::num::TryFromIntError;
use std::ops::{Deref, DerefMut};

/// An epoch represents a single valid state in the blockchain
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize,
)]
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

impl Deref for ChainEpoch {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChainEpoch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<u64> for ChainEpoch {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl AsMut<u64> for ChainEpoch {
    fn as_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl Borrow<u64> for ChainEpoch {
    fn borrow(&self) -> &u64 {
        &self.0
    }
}

impl BorrowMut<u64> for ChainEpoch {
    fn borrow_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

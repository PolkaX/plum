// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::time::Duration;

use anyhow::Result;

use plum_block::BeaconEntry;
use plum_hashing::blake2b_256;

use crate::drand::RandomBeacon;

/// MockBeacon assumes that filecoin rounds are 1:1 mapped with the beacon rounds.
pub struct MockBeacon {
    interval: Duration,
}

impl MockBeacon {
    /// Create a new MockBeacon.
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }

    /// Return the round time.
    pub fn round_time(&self) -> Duration {
        self.interval
    }

    fn entry_for_index(index: u64) -> BeaconEntry {
        let data = blake2b_256(index.to_be_bytes());
        BeaconEntry::new(index, data.to_vec())
    }
}

#[async_trait::async_trait]
impl RandomBeacon for MockBeacon {
    async fn entry(&self, round: u64) -> Result<BeaconEntry> {
        Ok(Self::entry_for_index(round))
    }

    fn verify_entry(&self, curr: &BeaconEntry, _prev: &BeaconEntry) -> Result<bool> {
        let oe = Self::entry_for_index(curr.round());
        Ok(oe.data() == curr.data())
    }

    fn max_beacon_round_for_epoch(&self, fil_epoch: i64) -> u64 {
        fil_epoch as u64
    }
}

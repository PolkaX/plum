// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::sync::Arc;

use anyhow::Result;
use grpcio::{ChannelBuilder, EnvBuilder};

use plum_block::BeaconEntry;
use plum_types::ChainEpoch;

use crate::proto::drand::{create_public, Public, PublicClient};

fn test() {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect("localhost:50051");
    let client = PublicClient::new(channel);
}

/// RandomBeacon represents a system that provides randomness to Lotus.
/// Other components interrogate the RandomBeacon to acquire randomness that's
/// valid for a specific chain epoch.
/// Also to verify beacon entries that have been posted on chain.
#[async_trait::async_trait]
pub trait RandomBeacon {
    /// Acquire a beacon entry from the drand network with the given round number.
    async fn entry(&self, round: u64) -> Result<BeaconEntry>;

    /// Verify a beacon against the previous.
    fn verify_entry(&self, curr: &BeaconEntry, prev: &BeaconEntry) -> Result<bool>;

    /// Calculates the maximum beacon round for the given filecoin epoch
    fn max_beacon_round_for_epoch(&self, fil_epoch: ChainEpoch) -> u64;
}

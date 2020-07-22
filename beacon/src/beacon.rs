// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use grpcio::{ChannelBuilder, EnvBuilder, LbPolicy};

use plum_block::BeaconEntry;
use plum_types::ChainEpoch;

use crate::proto::drand::{PublicClient, PublicRandRequest};

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

/// DrandBeacon connects Lotus with a drand network in order to provide
/// randomness to the system in a way that's aligned with Filecoin rounds/epochs.
///
/// We connect to drand peers via their public HTTP endpoints.
/// The peers are enumerated in the drandServers variable.
///
/// The root trust for the Drand chain is configured from build.DrandChain.
pub struct DrandBeacon {
    client: PublicClient,
    pubkey: String,

    interval: Duration,
    drand_gen_time: u64,
    fil_gen_time: u64,
    fil_round_time: u64,
}

/*
var DrandConfig = dtypes.DrandConfig{
    Servers: []string{
        "https://pl-eu.testnet.drand.sh",
        "https://pl-us.testnet.drand.sh",
        "https://pl-sin.testnet.drand.sh",
     },
    ChainInfoJSON: `{
        "public_key":"922a2e93828ff83345bae533f5172669a26c02dc76d6bf59c80892e12ab1455c229211886f35bb56af6d5bea981024df",
        "period":25,
        "genesis_time":1590445175,
        "hash":"138a324aa6540f93d0dad002aa89454b1bec2b6e948682cde6bd4db40f4b7c9b"
    }`,
}
*/
impl DrandBeacon {
    /// Create a new DrandBeacon with the config.
    pub fn new(genesis_ts: u64, interval: u64) -> Result<Self> {
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env)
            .load_balancing_policy(LbPolicy::RoundRobin)
            .connect("https://pl-eu.testnet.drand.sh");
        let client = PublicClient::new(channel);

        Ok(Self {
            client,
            pubkey: "922a2e93828ff83345bae533f5172669a26c02dc76d6bf59c80892e12ab1455c229211886f35bb56af6d5bea981024df".to_string(),
            interval: Duration::from_secs(25),
            drand_gen_time: 1590445175,
            fil_round_time: interval,
            fil_gen_time: genesis_ts,
        })
    }
}

#[async_trait::async_trait]
impl RandomBeacon for DrandBeacon {
    async fn entry(&self, round: u64) -> Result<BeaconEntry> {
        let mut public_rand_req = PublicRandRequest::new_();
        public_rand_req.round = round;
        let public_rand_resp = self.client.public_rand_async(&public_rand_req)?.await?;
        Ok(BeaconEntry::new(
            public_rand_resp.round,
            public_rand_resp.signature,
        ))
    }

    fn verify_entry(&self, curr: &BeaconEntry, prev: &BeaconEntry) -> Result<bool> {
        if prev.round() == 0 {
            return Ok(true);
        }

        // TODO
        Ok(false)
    }

    fn max_beacon_round_for_epoch(&self, fil_epoch: ChainEpoch) -> u64 {
        // TODO: sometimes the genesis time for filecoin is zero and this goes negative
        let latest_ts =
            fil_epoch as u64 * self.fil_round_time + self.fil_gen_time - self.fil_round_time;
        (latest_ts - self.drand_gen_time) / self.interval.as_secs()
    }
}

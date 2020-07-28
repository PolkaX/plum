// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::sync::Arc;

use anyhow::Result;
use bls::{PublicKey, Serialize};
use grpcio::{ChannelBuilder, EnvBuilder, LbPolicy};

use plum_block::BeaconEntry;
use plum_hashing::sha256;
use plum_types::ChainEpoch;

use crate::config::DrandConfig;
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
    pubkey: PublicKey,

    interval: u64, // time.Duration (i64)
    drand_gen_time: u64,
    fil_gen_time: u64,
    fil_round_time: u64,
}

impl DrandBeacon {
    /// Create a new DrandBeacon with the config.
    pub fn new(genesis_ts: u64, interval: u64, config: DrandConfig) -> Result<Self> {
        let addr = config.servers.join(",");

        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env)
            .load_balancing_policy(LbPolicy::RoundRobin)
            .connect(&addr);
        let client = PublicClient::new(channel);

        Ok(Self {
            client,
            pubkey: config.chain_info.public_key,
            interval: config.chain_info.period,
            drand_gen_time: config.chain_info.genesis_time,
            fil_round_time: interval,
            fil_gen_time: genesis_ts,
        })
    }

    fn verify_beacon_data(&self, round: u64, curr_sig: &[u8], prev_sig: &[u8]) -> Result<bool> {
        let mut message = Vec::with_capacity(prev_sig.len() + 8);
        message.extend_from_slice(prev_sig);
        message.extend_from_slice(&round.to_be_bytes());
        // H ( prevSig || currRound)
        let digest = sha256(message);
        // When signing with `BLS` privkey, the message will be hashed in `bls::PrivateKey::sign`,
        // so the message here needs to be hashed before the signature is verified.
        let digest = bls::hash(&digest);

        let sig = bls::Signature::from_bytes(curr_sig)?;
        Ok(bls::verify(&sig, &[digest], &[self.pubkey]))
    }
}

#[async_trait::async_trait]
impl RandomBeacon for DrandBeacon {
    async fn entry(&self, round: u64) -> Result<BeaconEntry> {
        let mut public_rand_req = PublicRandRequest::default();
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
        self.verify_beacon_data(curr.round(), curr.data(), prev.data())
    }

    fn max_beacon_round_for_epoch(&self, fil_epoch: ChainEpoch) -> u64 {
        // TODO: sometimes the genesis time for filecoin is zero and this goes negative
        let latest_ts =
            fil_epoch as u64 * self.fil_round_time + self.fil_gen_time - self.fil_round_time;
        (latest_ts - self.drand_gen_time) / self.interval
    }
}

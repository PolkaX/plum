// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use plum_address::Address;
use plum_bigint::{bigint_json, BigInt};
use plum_block::{BeaconEntry, BlockMsg, ElectionProof, Ticket};
use plum_message::SignedMessage;
use plum_sector::{PoStProof, SectorInfo, SectorSize};
use plum_tipset::TipsetKey;
use plum_types::ChainEpoch;

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

/// MethodGroup: Miner.
#[doc(hidden)]
#[async_trait::async_trait]
pub trait MinerApi: RpcClient {
    async fn miner_get_base_info(
        &self,
        addr: &Address,
        height: ChainEpoch,
        key: &TipsetKey,
    ) -> Result<Option<MiningBaseInfo>> {
        self.request(
            "MinerGetBaseInfo",
            vec![
                helper::serialize(addr),
                helper::serialize(&height),
                helper::serialize(key),
            ],
        )
        .await
    }

    async fn miner_create_block(&self, template: &BlockTemplate) -> Result<BlockMsg> {
        self.request("MinerCreateBlock", vec![helper::serialize(template)])
            .await
    }
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiningBaseInfo {
    #[serde(with = "bigint_json")]
    pub miner_power: BigInt,
    #[serde(with = "bigint_json")]
    pub network_power: BigInt,
    pub sectors: Vec<SectorInfo>,
    pub worker_key: Address,
    pub sector_size: SectorSize,
    pub prev_beacon_entry: BeaconEntry,
    pub beacon_entries: Vec<BeaconEntry>,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockTemplate {
    pub miner: Address,
    pub parents: TipsetKey,
    pub ticket: Ticket,
    pub eproof: ElectionProof,
    pub beacon_values: Vec<BeaconEntry>,
    pub messages: Vec<SignedMessage>,
    pub epoch: ChainEpoch,
    pub timestamp: u64,
    #[serde(rename = "WinningPoStProof")]
    pub winning_post_proof: Vec<PoStProof>,
}

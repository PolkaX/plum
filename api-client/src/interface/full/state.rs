// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use cid::Cid;
use plum_actor::{market, miner, power};
use plum_address::Address;
use plum_bigint::{bigint_json, BigInt, BigIntWrapper};
use plum_bitfield::BitField;
use plum_message::{MessageReceipt, UnsignedMessage};
use plum_sector::SectorNumber;
use plum_tipset::{Tipset, TipsetKey};
use plum_types::{Actor, ChainEpoch};
use plum_vm::ExecutionResult;

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

/// MethodGroup: State.
/// The State methods are used to query, inspect, and interact with chain state.
#[doc(hidden)]
#[async_trait::async_trait]
pub trait StateApi: RpcClient {
    // if tipset is nil, we'll use heaviest
    async fn state_call(&self, msg: &UnsignedMessage, key: &TipsetKey) -> Result<InvocResult> {
        self.request(
            "StateCall",
            vec![helper::serialize(msg), helper::serialize(key)],
        )
        .await
    }

    async fn state_replay(&self, key: &TipsetKey, cid: &Cid) -> Result<InvocResult> {
        self.request(
            "StateReplay",
            vec![helper::serialize(key), helper::serialize(cid)],
        )
        .await
    }

    async fn state_get_actor(&self, addr: &Address, key: &TipsetKey) -> Result<Actor> {
        self.request(
            "StateGetActor",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    /*
    async fn state_read_state(&self, actor: &Actor, key: &TipsetKey) -> Result<ActorState> {
        self.request(
            "StateReadState",
            vec![helper::serialize(actor), helper::serialize(key)],
        )
        .await
    }
    */

    async fn state_list_messages(
        &self,
        msg: &UnsignedMessage,
        key: &TipsetKey,
        height: ChainEpoch,
    ) -> Result<Vec<Cid>> {
        self.request(
            "StateListMessages",
            vec![
                helper::serialize(msg),
                helper::serialize(key),
                helper::serialize(&height),
            ],
        )
        .await
    }

    async fn state_network_name(&self) -> Result<String> {
        self.request("StateNetworkName", vec![]).await
    }

    async fn state_miner_sectors(
        &self,
        addr: &Address,
        filter: &BitField,
        filter_out: bool,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        self.request(
            "StateMinerSectors",
            vec![
                helper::serialize(addr),
                helper::serialize(filter),
                helper::serialize(&filter_out),
                helper::serialize(key),
            ],
        )
        .await
    }

    async fn state_miner_proving_set(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        self.request(
            "StateMinerProvingSet",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_proving_deadline(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<miner::DeadlineInfo> {
        self.request(
            "StateMinerProvingDeadline",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_power(&self, addr: &Address, key: &TipsetKey) -> Result<MinerPower> {
        self.request(
            "StateMinerPower",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_info(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<Option<miner::MinerInfo>> {
        self.request(
            "StateMinerInfo",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_deadlines(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<miner::Deadlines> {
        self.request(
            "StateMinerDeadlines",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_faults(&self, addr: &Address, key: &TipsetKey) -> Result<BitField> {
        self.request(
            "StateMinerFaults",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    // Returns all non-expired Faults that occur within lookback epochs of the given tipset
    async fn state_all_miner_faults(
        &self,
        lookback: ChainEpoch,
        key: &TipsetKey,
    ) -> Result<Vec<Fault>> {
        self.request(
            "StateAllMinerFaults",
            vec![helper::serialize(&lookback), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_recoveries(&self, addr: &Address, key: &TipsetKey) -> Result<BitField> {
        self.request(
            "StateMinerRecoveries",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_initial_pledge_collateral(
        &self,
        addr: &Address,
        sector_number: SectorNumber,
        key: &TipsetKey,
    ) -> Result<BigInt> {
        let bigint: BigIntWrapper = self
            .request(
                "StateMinerInitialPledgeCollateral",
                vec![
                    helper::serialize(addr),
                    helper::serialize(&sector_number),
                    helper::serialize(key),
                ],
            )
            .await?;
        Ok(bigint.into_inner())
    }

    async fn state_miner_available_balance(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<BigInt> {
        let bigint: BigIntWrapper = self
            .request(
                "StateMinerAvailableBalance",
                vec![helper::serialize(addr), helper::serialize(key)],
            )
            .await?;
        Ok(bigint.into_inner())
    }

    async fn state_sector_pre_commit_info(
        &self,
        addr: &Address,
        sector_number: SectorNumber,
        key: &TipsetKey,
    ) -> Result<miner::SectorPreCommitOnChainInfo> {
        self.request(
            "StateSectorPreCommitInfo",
            vec![
                helper::serialize(addr),
                helper::serialize(&sector_number),
                helper::serialize(key),
            ],
        )
        .await
    }

    async fn state_pledge_collateral(&self, key: &TipsetKey) -> Result<BigInt> {
        let bigint: BigIntWrapper = self
            .request("StatePledgeCollateral", vec![helper::serialize(key)])
            .await?;
        Ok(bigint.into_inner())
    }

    async fn state_wait_msg(&self, cid: &Cid) -> Result<MsgLookup> {
        self.request("StateWaitMsg", vec![helper::serialize(cid)])
            .await
    }

    async fn state_search_msg(&self, cid: &Cid) -> Result<MsgLookup> {
        self.request("StateSearchMsg", vec![helper::serialize(cid)])
            .await
    }

    async fn state_list_miners(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        self.request("StateListMiners", vec![helper::serialize(key)])
            .await
    }

    async fn state_list_actors(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        self.request("StateListActors", vec![helper::serialize(key)])
            .await
    }

    async fn state_market_balance(&self, addr: &Address, key: &TipsetKey) -> Result<MarketBalance> {
        self.request(
            "StateMarketBalance",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_market_participants(
        &self,
        key: &TipsetKey,
    ) -> Result<HashMap<String, MarketBalance>> {
        self.request("StateMarketParticipants", vec![helper::serialize(key)])
            .await
    }

    async fn state_market_deals(&self, key: &TipsetKey) -> Result<HashMap<String, MarketDeal>> {
        self.request("StateMarketDeals", vec![helper::serialize(key)])
            .await
    }

    async fn state_market_storage_deal(&self, deal_id: u64, key: &TipsetKey) -> Result<MarketDeal> {
        self.request(
            "StateMarketStorageDeal",
            vec![helper::serialize(&deal_id), helper::serialize(key)],
        )
        .await
    }

    async fn state_lookup_id(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        self.request(
            "StateLookupID",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_account_key(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        self.request(
            "StateAccountKey",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_changed_actors(&self, old: &Cid, new: &Cid) -> Result<HashMap<String, Actor>> {
        self.request(
            "StateChangedActors",
            vec![helper::serialize(old), helper::serialize(new)],
        )
        .await
    }

    async fn state_get_receipt(&self, cid: &Cid, key: &TipsetKey) -> Result<MessageReceipt> {
        self.request(
            "StateGetReceipt",
            vec![helper::serialize(cid), helper::serialize(key)],
        )
        .await
    }

    async fn state_miner_sector_count(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<MinerSectors> {
        self.request(
            "StateMinerSectorCount",
            vec![helper::serialize(addr), helper::serialize(key)],
        )
        .await
    }

    async fn state_compute(
        &self,
        height: ChainEpoch,
        msgs: &[UnsignedMessage],
        key: &TipsetKey,
    ) -> Result<ComputeStateOutput> {
        self.request(
            "StateCompute",
            vec![
                helper::serialize(&height),
                helper::serialize(&msgs),
                helper::serialize(key),
            ],
        )
        .await
    }
}

/*
///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActorState {
    #[serde(with = "bigint_json")]
    pub balance: BigInt,
    // pub state: interface{},
}
*/

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChainSectorInfo {
    pub info: miner::SectorOnChainInfo,
    pub id: SectorNumber,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinerPower {
    pub miner_power: power::Claim,
    pub total_power: power::Claim,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinerSectors {
    pub pset: u64,
    pub sset: u64,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fault {
    pub miner: Address,
    pub epoch: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MsgLookup {
    pub receipt: MessageReceipt,
    #[serde(rename = "TipSet")]
    pub tipset: Tipset,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketBalance {
    #[serde(with = "bigint_json")]
    pub escrow: BigInt,
    #[serde(with = "bigint_json")]
    pub locked: BigInt,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketDeal {
    pub proposal: market::DealProposal,
    pub state: market::DealState,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvocResult {
    pub msg: UnsignedMessage,
    pub msg_rct: MessageReceipt,
    pub internal_executions: Vec<ExecutionResult>,
    pub error: String,
    pub duration: i64, // time.Duration is a alias of i64 in golang
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComputeStateOutput {
    pub root: Cid,
    pub trace: Vec<InvocResult>,
}

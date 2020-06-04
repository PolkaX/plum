// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use cid::Cid;
use plum_address::Address;
use plum_bigint::bigint_json;
use plum_bitfield::BitField;
use plum_peerid::PeerId;
use plum_sector::{RegisteredProof, SectorNumber, SectorSize};
use plum_types::{ChainEpoch, DealId, DealWeight, TokenAmount};

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinerInfo {
    /// Account that owns this miner.
    /// - Income and returned collateral are paid to this address.
    /// - This address is also allowed to change the worker address for the miner.
    pub owner: Address, // Must be an ID-address.
    /// Worker account for this miner.
    /// The associated pubkey-type address is used to sign blocks and messages on behalf of this miner.
    pub worker: Address, // Must be an ID-address.

    pub pending_worker_key: WorkerKeyChange,

    /// Libp2p identity that should be used when connecting to this miner.
    #[serde(with = "plum_peerid")]
    pub peer_id: PeerId,

    /// The proof type used by this miner for sealing sectors.
    pub seal_proof_type: RegisteredProof,

    /// Amount of space in each sector committed by this miner.
    /// This is computed from the proof type and represented here redundantly.
    pub sector_size: SectorSize,

    /// The number of sectors in each Window PoSt partition (proof).
    /// This is computed from the proof type and represented here redundantly.
    #[serde(rename = "WindowPoStPartitionSectors")]
    pub window_post_partition_sectors: u64,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorkerKeyChange {
    pub new_worker: Address, // Must be an ID address
    pub effective_at: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorPreCommitInfo {
    pub registered_proof: RegisteredProof,
    pub sector_number: SectorNumber,
    // CommR
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
    pub seal_rand_epoch: ChainEpoch,
    #[serde(rename = "DealIDs")]
    pub deal_ids: Vec<DealId>,
    // Sector expiration
    pub expiration: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorPreCommitOnChainInfo {
    pub info: SectorPreCommitInfo,
    #[serde(with = "bigint_json")]
    pub pre_commit_deposit: TokenAmount,
    pub pre_commit_epoch: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorOnChainInfo {
    pub info: SectorPreCommitInfo,
    // Epoch at which SectorProveCommit is accepted
    pub activation_epoch: ChainEpoch,
    // Integral of active deals over sector lifetime
    #[serde(with = "bigint_json")]
    pub deal_weight: DealWeight,
    // Integral of active verified deals over sector lifetime
    #[serde(with = "bigint_json")]
    pub verified_deal_weight: DealWeight,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Deadlines {
    // A bitfield of sector numbers due at each deadline.
    // The sectors for each deadline are logically grouped into sequential partitions for proving.
    pub due: BitField, // [WPoStPeriodDeadlines]*abi.BitField
}

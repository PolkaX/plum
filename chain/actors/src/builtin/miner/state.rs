use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use cid::Cid;
use plum_address::Address;
use plum_types::{chain_epoch::ChainEpoch, DealId, DealWeight, PeerId, TokenAmount};

use crate::abi::bitfield::BitField;
use crate::abi::sector::{SectorNumber, SectorSize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct State {
    /// Map, HAMT[SectorNumber]SectorPreCommitOnChainInfo
    pub precommitted_sectors: Cid,
    /// Array, AMT[]SectorOnChainInfo (sparse)
    pub sectors: Cid,
    pub fault_set: BitField,
    /// Array, AMT[]SectorOnChainInfo (sparse)
    pub proving_set: Cid,
    pub info: MinerInfo,
    pub post_state: PoStState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct MinerInfo {
    /// Account that owns this miner.
    /// - Income and returned collateral are paid to this address.
    /// - This address is also allowed to change the worker address for the miner.
    ///
    /// Must be an ID-address.
    #[serde(with = "plum_address::address_cbor")]
    pub owner: Address,
    /// Worker account for this miner. The associated pubkey-type address is used
    /// to sign blocks and messages on behalf of this miner. Must be an ID-address.
    #[serde(with = "plum_address::address_cbor")]
    pub worker: Address,
    pub pending_worker_key: Option<WorkerKeyChange>,
    /// Libp2p identity that should be used when connecting to this miner.
    pub peer_id: PeerId,
    /// Amount of space in each sector committed to the network by this miner.
    pub sector_size: SectorSize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct PoStState {
    /// Epoch that starts the current proving period
    pub proving_period_start: ChainEpoch,
    /// Number of surprised post challenges that have been failed since last
    /// successful PoSt. Indicates that the claimed storage power may not
    /// actually be proven. Recovery can proceed by submitting a correct response
    /// to a subsequent PoSt challenge, up until the limit of number of
    /// consecutive failures.
    pub num_consecutive_failures: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct WorkerKeyChange {
    /// Must be an ID address
    #[serde(with = "plum_address::address_cbor")]
    pub new_worker: Address,
    pub effective_at: ChainEpoch,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorPreCommitInfo {
    pub sector: SectorNumber,
    /// CommR
    pub sealed_cid: Cid,
    pub seal_epoch: ChainEpoch,
    pub deal_ids: Vec<DealId>,
    /// Sector expiration
    pub expiration: ChainEpoch,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorPreCommitOnChainInfo {
    pub info: SectorPreCommitInfo,
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub precommit_deposit: TokenAmount,
    pub precommit_epoch: ChainEpoch,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorOnChainInfo {
    pub info: SectorPreCommitInfo,
    /// Epoch at which SectorProveCommit is accepted
    pub activation_epoch: ChainEpoch,
    /// Integral of active deals over sector lifetime, 0 if CommittedCapacity sector
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub deal_weight: DealWeight,
    /// Fixed pledge collateral requirement determined at activation
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub pledge_requirement: TokenAmount,
    pub declared_fault_epoch: ChainEpoch,
    pub declared_fault_duration: ChainEpoch,
}

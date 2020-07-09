// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use plum_address::Address;
use plum_bigint::bigint_json;
use plum_bitfield::BitField;
use plum_peerid::PeerId;
use plum_sector::{RegisteredSealProof, SectorNumber, SectorSize};
use plum_types::{ChainEpoch, DealId, DealWeight, TokenAmount};

// Balance of Miner Actor should be greater than or equal to
// the sum of pre_commit_deposits and locked_funds.
// Excess balance as computed by st.GetAvailableBalance will be
// withdrawable or usable for pre-commit deposit or pledge lock-up.
///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    // Information not related to sectors.
    pub info: MinerInfo,
    #[serde(with = "bigint_json")]
    pub pre_commit_deposits: TokenAmount,
    #[serde(with = "bigint_json")]
    pub locked_funds: TokenAmount,
    pub vesting_funds: Cid,

    // sectors that have been pre-committed but not yet proven.
    pub pre_committed_sectors: Cid, // Map, HAMT[SectorNumber]SectorPreCommitOnChainInfo

    // Information for all proven and not-yet-expired sectors.
    pub sectors: Cid, // Array, AMT[SectorNumber]SectorOnChainInfo (sparse)

    // The first epoch in this miner's current proving period. This is the first epoch in which a PoSt for a
    // partition at the miner's first deadline may arrive. Alternatively, it is after the last epoch at which
    // a PoSt for the previous window is valid.
    // Always greater than zero, his may be greater than the current epoch for genesis miners in the first
    // WPoStProvingPeriod epochs of the chain; the epochs before the first proving period starts are exempt from Window
    // PoSt requirements.
    // Updated at the end of every period by a power actor cron event.
    pub proving_period_start: ChainEpoch,

    // Sector numbers prove-committed since period start, to be added to deadlines at next proving period boundary.
    pub new_sectors: BitField,

    // Sector numbers indexed by expiry epoch (which are on proving period boundaries).
    // Invariant: Keys(sectors) == union(sector_expirations.Values())
    pub sector_expirations: Cid, // Array, AMT[ChainEpoch]Bitfield

    // The sector numbers due for PoSt at each deadline in the current proving period, frozen at period start.
    // New sectors are added and expired ones removed at proving period boundary.
    // faults are not subtracted from this in state, but on the fly.
    pub deadlines: Cid,

    // All currently known faulty sectors, mutated eagerly.
    // These sectors are exempt from inclusion in PoSt.
    pub faults: BitField,

    // Faulty sector numbers indexed by the start epoch of the proving period in which detected.
    // Used to track fault durations for eventual sector termination.
    // At most 14 entries, b/c sectors faulty longer expire.
    // Invariant: faults == union(fault_epochs.Values())
    pub fault_epochs: Cid, // AMT[ChainEpoch]Bitfield

    // Faulty sectors that will recover when next included in a valid PoSt.
    // Invariant: recoveries ⊆ faults.
    pub recoveries: BitField,

    // Records successful PoSt submission in the current proving period by partition number.
    // The presence of a partition number indicates on-time PoSt received.
    pub post_submissions: BitField,
}

impl minicbor::Encode for State {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(14)?
            .encode(&self.info)?
            .encode(&plum_bigint::BigIntRefWrapper::from(
                &self.pre_commit_deposits,
            ))?
            .encode(&plum_bigint::BigIntRefWrapper::from(&self.locked_funds))?
            .encode(&self.vesting_funds)?
            .encode(&self.pre_committed_sectors)?
            .encode(&self.sectors)?
            .encode(&self.proving_period_start)?
            .encode(&self.new_sectors)?
            .encode(&self.sector_expirations)?
            .encode(&self.deadlines)?
            .encode(&self.faults)?
            .encode(&self.fault_epochs)?
            .encode(&self.recoveries)?
            .encode(&self.post_submissions)?
            .ok()
    }
}

impl<'b> minicbor::Decode<'b> for State {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(14));
        Ok(State {
            info: d.decode::<MinerInfo>()?,
            pre_commit_deposits: d.decode::<plum_bigint::BigIntWrapper>()?.into_inner(),
            locked_funds: d.decode::<plum_bigint::BigIntWrapper>()?.into_inner(),
            vesting_funds: d.decode::<Cid>()?,
            pre_committed_sectors: d.decode::<Cid>()?,
            sectors: d.decode::<Cid>()?,
            proving_period_start: d.decode::<ChainEpoch>()?,
            new_sectors: d.decode::<BitField>()?,
            sector_expirations: d.decode::<Cid>()?,
            deadlines: d.decode::<Cid>()?,
            faults: d.decode::<BitField>()?,
            fault_epochs: d.decode::<Cid>()?,
            recoveries: d.decode::<BitField>()?,
            post_submissions: d.decode::<BitField>()?,
        })
    }
}
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
    pub seal_proof_type: RegisteredSealProof,

    /// Amount of space in each sector committed by this miner.
    /// This is computed from the proof type and represented here redundantly.
    pub sector_size: SectorSize,

    /// The number of sectors in each Window PoSt partition (proof).
    /// This is computed from the proof type and represented here redundantly.
    #[serde(rename = "WindowPoStPartitionSectors")]
    pub window_post_partition_sectors: u64,
}

impl minicbor::Encode for MinerInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(7)?
            .encode(&self.owner)?
            .encode(&self.worker)?
            .encode(&self.pending_worker_key)?
            .encode(plum_peerid::PeerIdRefWrapper::from(&self.peer_id))?
            .encode(&self.seal_proof_type)?
            .encode(&self.sector_size)?
            .encode(&self.window_post_partition_sectors)?
            .ok()
    }
}

impl<'b> minicbor::Decode<'b> for MinerInfo {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(7));
        Ok(MinerInfo {
            owner: d.decode::<Address>()?,
            worker: d.decode::<Address>()?,
            pending_worker_key: d.decode::<WorkerKeyChange>()?,
            peer_id: d.decode::<plum_peerid::PeerIdWrapper>()?.into_inner(),
            seal_proof_type: d.decode::<RegisteredSealProof>()?,
            sector_size: d.decode::<SectorSize>()?,
            window_post_partition_sectors: d.decode::<u64>()?,
        })
    }
}

///
#[doc(hidden)]
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, minicbor::Encode, minicbor::Decode,
)]
#[serde(rename_all = "PascalCase")]
#[cbor(array)]
pub struct WorkerKeyChange {
    #[n(0)]
    pub new_worker: Address, // Must be an ID address
    #[n(1)]
    pub effective_at: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, minicbor::Encode, minicbor::Decode,
)]
#[serde(rename_all = "PascalCase")]
#[cbor(array)]
pub struct SectorPreCommitInfo {
    #[n(0)]
    pub registered_proof: RegisteredSealProof,
    #[n(1)]
    pub sector_number: SectorNumber,
    /// CommR
    #[n(2)]
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
    #[n(3)]
    pub seal_rand_epoch: ChainEpoch,
    #[n(4)]
    #[serde(rename = "DealIDs")]
    pub deal_ids: Vec<DealId>,
    /// Sector expiration
    #[n(5)]
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

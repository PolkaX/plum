// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use serde::{Deserialize, Serialize};

use plum_piece::UnpaddedPieceSize;
use plum_sector::SectorNumber;
use plum_types::{ChainEpoch, DealId, Randomness};

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorInfo {
    pub sector_id: SectorNumber,
    pub state: SectorState,
    pub comm_d: Cid,
    pub comm_r: Cid,
    #[serde(with = "plum_bytes")]
    pub proof: Vec<u8>,
    pub deals: Vec<DealId>,
    pub ticket: SealTicket,
    pub seed: SealSeed,
    pub retries: u64,

    pub last_err: String,

    pub log: Vec<SectorLog>,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorLog {
    pub kind: String,
    pub timestamp: u64,
    pub trace: String,
    pub message: String,
}

///
#[doc(hidden)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealedRef {
    #[serde(rename = "SectorID")]
    pub sector_id: SectorNumber,
    pub offset: u64,
    pub size: UnpaddedPieceSize,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealedRefs {
    pub refs: Vec<SealedRef>,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealTicket {
    #[serde(with = "plum_bytes")]
    pub value: Randomness,
    pub epoch: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealSeed {
    #[serde(with = "plum_bytes")]
    pub value: Randomness,
    pub epoch: ChainEpoch,
}

///
pub type SectorState = String;

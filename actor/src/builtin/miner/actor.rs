// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_bitfield::BitField;
use plum_sector::RegisteredProof;

pub enum CronEventType {
    WindowedPoStExpiration,
    WorkerKeyChange,
    PreCommitExpiry,
    SectorExpiry,
    TempFault,
}

pub struct CronEventPayload {
    pub event_type: CronEventType,
    pub sectors: Option<BitField>,
    pub registered_proof: RegisteredProof,
}

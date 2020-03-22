use crate::abi::bitfield::BitField;
use crate::abi::sector::RegisteredProof;

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

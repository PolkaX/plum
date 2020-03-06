use crate::error::AbiError;
use cid::*;
use plum_bigint::BigInt;
use types::chain_epoch::ChainEpoch;

pub type ActorID = u64;
pub type SectorNumber = u64;
pub type SectorSize = u64;

// The unit of sector weight (power-epochs)
pub type SectorWeight = BigInt;

// The unit of storage power (measured in bytes)
pub type StoragePower = BigInt;

type SealRandomness = [u8; 32];
type InteractiveSealRandomness = [u8; 32];
type ChallengeTicketsCommitment = [u8; 32];
type PoStRandomness = [u8; 32];
type PartialTicket = [u8; 32];

#[derive(Debug, Clone)]
pub struct SectorID {
    pub miner: ActorID,
    pub number: SectorNumber,
}

#[derive(Debug, Clone)]
pub struct PoStProof {
    //<curve, system> {
    pub proof_bytes: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct PrivatePoStCandidateProof {
    pub externalized: [u8; 32],
}

#[derive(Debug, Clone)]
pub enum RegisteredProof {
    StackedDRG32GiBSeal = 1,
    StackedDRG32GiBPoSt,
    StackedDRG2KiBSeal,
    StackedDRG2KiBPoSt,
    StackedDRG8MiBSeal,
    StackedDRG8MiBPoSt,
    StackedDRG512MiBSeal,
    StackedDRG512MiBPoSt,
}

// OnChainSealVerifyInfo is the structure of information that must be sent with
// a message to commit a sector. Most of this information is not needed in the
// state tree but will be verified in sm.CommitSector. See SealCommitment for
// data stored on the state tree for each sector.
#[derive(Debug, Clone)]
pub struct OnChainSealVerifyInfo {
    pub sealed_cid: Cid,               // CommR
    pub interactive_epoch: ChainEpoch, // Used to derive the interactive PoRep challenge.
    pub registered_proof: RegisteredProof,
    pub proof: Vec<u8>,
    pub deal_id: u64,
    pub sector: SectorNumber,
    pub seal_rand_epoch: ChainEpoch, // Used to tie the seal to a chain.
}

// SealVerifyInfo is the structure of all the information a verifier
// needs to verify a Seal.
#[derive(Debug, Clone)]
pub struct SealVerifyInfo {
    pub sector: SectorID,
    pub on_chain: OnChainSealVerifyInfo,
    pub seal_randomness: SealRandomness,
    pub interactive_randomness: InteractiveSealRandomness,
    pub unsealed_cid: Cid, // CommD
}

#[derive(Debug, Clone)]
pub struct PoStCandidate {
    pub partial_ticket: PartialTicket, // Optional —  will eventually be omitted for SurprisePoSt verification, needed for now.
    pub private_proof: PrivatePoStCandidateProof, // Optional — should be ommitted for verification.
    pub sector_id: SectorID,
    pub challenge_index: u64,
}

#[derive(Debug, Clone)]
pub struct OnChainPoStVerifyInfo {
    pub candidates: Vec<PoStCandidate>, // each PoStCandidate has its own RegisteredProof
    pub proofs: Vec<PoStProof>,         // each PoStProof has its own RegisteredProof
}

#[derive(Debug, Clone)]
pub struct PoStVerifyInfo {
    pub post_randomness: PoStRandomness,
    pub candidates: Vec<PoStCandidate>, // From OnChain*PoStVerifyInfo
    pub proofs: Vec<PoStProof>,
    pub eligible_sectors: Vec<SectorInfo>,
    pub prover: ActorID, // used to derive 32-byte prover ID
    pub challenge_count: u64,
}

#[derive(Debug, Clone)]
pub struct SectorInfo {
    pub sector_number: SectorNumber,
    pub sealed_cid: Cid, // CommR
}

#[derive(Debug, Clone)]
pub struct OnChainElectionPoStVerifyInfo {
    pub candidates: Vec<PoStCandidate>, // each PoStCandidate has its own RegisteredProof
    pub proofs: Vec<PoStProof>,         // each PoStProof has its own RegisteredProof
    pub randomness: PoStRandomness,
}

pub fn new_storage_power(num: u64) -> StoragePower {
    StoragePower::from(num)
}

pub fn sector_size(registered_proof: RegisteredProof) -> std::result::Result<SectorSize, AbiError> {
    match registered_proof {
        RegisteredProof::StackedDRG32GiBSeal | RegisteredProof::StackedDRG32GiBPoSt => Ok(32 << 30),
        RegisteredProof::StackedDRG2KiBSeal | RegisteredProof::StackedDRG2KiBPoSt => Ok(2 << 10),
        RegisteredProof::StackedDRG8MiBSeal | RegisteredProof::StackedDRG8MiBPoSt => Ok(8 << 20),
        RegisteredProof::StackedDRG512MiBSeal | RegisteredProof::StackedDRG512MiBPoSt => {
            Ok(512 << 20)
        }
    }
}

// RegisteredPoStProof produces the PoSt-specific RegisteredProof corresponding
// to the receiving RegisteredProof.
pub fn registered_seal_proof(
    registered_proof: u64,
) -> std::result::Result<RegisteredProof, AbiError> {
    match registered_proof {
        1 | 2 => Ok(RegisteredProof::StackedDRG32GiBSeal),
        3 | 4 => Ok(RegisteredProof::StackedDRG2KiBSeal),
        5 | 6 => Ok(RegisteredProof::StackedDRG8MiBSeal),
        7 | 8 => Ok(RegisteredProof::StackedDRG512MiBSeal),
        _ => Err(AbiError::Unsupported),
    }
}

pub fn registered_post_proof(
    registered_proof: u64,
) -> std::result::Result<RegisteredProof, AbiError> {
    match registered_proof {
        1 | 2 => Ok(RegisteredProof::StackedDRG32GiBPoSt),
        3 | 4 => Ok(RegisteredProof::StackedDRG2KiBPoSt),
        5 | 6 => Ok(RegisteredProof::StackedDRG8MiBPoSt),
        7 | 8 => Ok(RegisteredProof::StackedDRG512MiBPoSt),
        _ => Err(AbiError::Unsupported),
    }
}

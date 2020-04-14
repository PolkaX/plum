use enum_repr_derive::TryFrom;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use cid::Cid;

use plum_bigint::BigInt;
use plum_hash::H256;
use plum_types::{ActorId, ChainEpoch, SectorNumber, SectorSize};

use super::serde_helper;

// The unit of sector weight (power-epochs)
pub type SectorWeight = BigInt;

// The unit of storage power (measured in bytes)
pub type StoragePower = BigInt;

pub type Randomness = H256;
pub type ChallengeTicketsCommitment = H256;
pub type PartialTicket = H256;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorId {
    pub miner: ActorId,
    pub number: SectorNumber,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct PoStProof {
    pub proof: RegisteredProof,
    #[serde(with = "serde_bytes")]
    pub proof_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct PrivatePoStCandidateProof {
    pub proof: RegisteredProof,
    #[serde(with = "serde_bytes")]
    pub externalized: Vec<u8>,
}

#[repr(usize)]
#[derive(TryFrom, Debug, Clone, Copy, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum RegisteredProof {
    StackedDRG32GiBSeal = 1,
    StackedDRG32GiBPoSt = 2,
    StackedDRG2KiBSeal = 3,
    StackedDRG2KiBPoSt = 4,
    StackedDRG8MiBSeal = 5,
    StackedDRG8MiBPoSt = 6,
    StackedDRG512MiBSeal = 7,
    StackedDRG512MiBPoSt = 8,
}

impl RegisteredProof {
    /// `registered_post_proof` produces the PoSt-specific RegisteredProof corresponding
    /// to the receiving RegisteredProof.
    pub fn registered_post_proof(self) -> RegisteredProof {
        match self {
            RegisteredProof::StackedDRG32GiBSeal | RegisteredProof::StackedDRG32GiBPoSt => {
                RegisteredProof::StackedDRG32GiBPoSt
            }
            RegisteredProof::StackedDRG2KiBSeal | RegisteredProof::StackedDRG2KiBPoSt => {
                RegisteredProof::StackedDRG2KiBPoSt
            }
            RegisteredProof::StackedDRG8MiBSeal | RegisteredProof::StackedDRG8MiBPoSt => {
                RegisteredProof::StackedDRG8MiBPoSt
            }
            RegisteredProof::StackedDRG512MiBSeal | RegisteredProof::StackedDRG512MiBPoSt => {
                RegisteredProof::StackedDRG512MiBPoSt
            }
        }
    }

    /// `registered_seal_proof` produces the PoSt-specific RegisteredProof corresponding
    /// to the receiving RegisteredProof.
    pub fn registered_seal_proof(self) -> RegisteredProof {
        match self {
            RegisteredProof::StackedDRG32GiBSeal | RegisteredProof::StackedDRG32GiBPoSt => {
                RegisteredProof::StackedDRG32GiBSeal
            }
            RegisteredProof::StackedDRG2KiBSeal | RegisteredProof::StackedDRG2KiBPoSt => {
                RegisteredProof::StackedDRG2KiBSeal
            }
            RegisteredProof::StackedDRG8MiBSeal | RegisteredProof::StackedDRG8MiBPoSt => {
                RegisteredProof::StackedDRG8MiBSeal
            }
            RegisteredProof::StackedDRG512MiBSeal | RegisteredProof::StackedDRG512MiBPoSt => {
                RegisteredProof::StackedDRG512MiBSeal
            }
        }
    }

    pub fn sector_size(self) -> SectorSize {
        match self {
            RegisteredProof::StackedDRG32GiBSeal | RegisteredProof::StackedDRG32GiBPoSt => 32 << 30,
            RegisteredProof::StackedDRG2KiBSeal | RegisteredProof::StackedDRG2KiBPoSt => 2 << 10,
            RegisteredProof::StackedDRG8MiBSeal | RegisteredProof::StackedDRG8MiBPoSt => 8 << 20,
            RegisteredProof::StackedDRG512MiBSeal | RegisteredProof::StackedDRG512MiBPoSt => {
                512 << 20
            }
        }
    }
}

// OnChainSealVerifyInfo is the structure of information that must be sent with
// a message to commit a sector. Most of this information is not needed in the
// state tree but will be verified in sm.CommitSector. See SealCommitment for
// data stored on the state tree for each sector.
#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct OnChainSealVerifyInfo {
    pub sealed_cid: Cid,               // CommR
    pub interactive_epoch: ChainEpoch, // Used to derive the interactive PoRep challenge.
    pub registered_proof: RegisteredProof,
    #[serde(with = "serde_bytes")]
    pub proof: Vec<u8>,
    pub deal_id: Vec<u64>,
    pub sector: SectorNumber,
    pub seal_rand_epoch: ChainEpoch, // Used to tie the seal to a chain.
}

// SealVerifyInfo is the structure of all the information a verifier
// needs to verify a Seal.
#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SealVerifyInfo {
    pub sector: SectorId,
    pub on_chain: OnChainSealVerifyInfo,
    #[serde(with = "plum_hash::h256_cbor")]
    pub seal_randomness: Randomness,
    #[serde(with = "plum_hash::h256_cbor")]
    pub interactive_randomness: Randomness,
    pub unsealed_cid: Cid, // CommD
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct PoStCandidate {
    pub proof: RegisteredProof,
    /// Optional —  will eventually be omitted for SurprisePoSt verification, needed for now.
    #[serde(with = "plum_hash::h256_cbor::option")]
    pub partial_ticket: Option<PartialTicket>,
    /// Optional — should be ommitted for verification.
    #[serde(with = "serde_helper::option_prev_post_candidate_proof")]
    pub private_proof: Option<PrivatePoStCandidateProof>,
    pub sector_id: SectorId,
    pub challenge_index: u64,
}

impl PoStCandidate {
    pub fn new(proof: RegisteredProof, sector_id: SectorId, challenge_index: u64) -> Self {
        PoStCandidate {
            proof,
            partial_ticket: None,
            private_proof: None,
            sector_id,
            challenge_index,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct OnChainPoStVerifyInfo {
    /// each PoStCandidate has its own RegisteredProof
    pub candidates: Vec<PoStCandidate>,
    /// each PoStProof has its own RegisteredProof
    pub proofs: Vec<PoStProof>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PoStVerifyInfo {
    pub post_randomness: Randomness,
    /// From OnChain*PoStVerifyInfo
    pub candidates: Vec<PoStCandidate>,
    pub proofs: Vec<PoStProof>,
    pub eligible_sectors: Vec<SectorInfo>,
    /// used to derive 32-byte prover ID
    pub prover: ActorId,
    pub challenge_count: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorInfo {
    /// RegisteredProof used when sealing - needs to be mapped to PoSt registered proof when used to verify a PoSt
    pub proof: RegisteredProof,
    pub sector_number: SectorNumber,
    /// CommR
    pub sealed_cid: Cid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct OnChainElectionPoStVerifyInfo {
    /// each PoStCandidate has its own RegisteredProof
    pub candidates: Vec<PoStCandidate>,
    /// each PoStProof has its own RegisteredProof
    pub proofs: Vec<PoStProof>,
    #[serde(with = "plum_hash::h256_cbor")]
    pub randomness: Randomness,
}

pub fn new_storage_power(num: u64) -> StoragePower {
    StoragePower::from(num)
}

#[test]
fn test_cbor() {
    use serde::{de::DeserializeOwned, Serialize};
    use std::fmt::Debug;

    fn asset_cbor<T: Serialize + DeserializeOwned + Eq + Debug>(obj: T, expect: Vec<u8>) {
        let v = serde_cbor::to_vec(&obj).unwrap();
        assert_eq!(v, expect);
        let out: T = serde_cbor::from_slice(&v).unwrap();
        assert_eq!(obj, out);
    }
    let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
        .parse()
        .unwrap();

    // sector id
    let id = SectorId {
        miner: 100,
        number: 100,
    };
    asset_cbor(id, vec![130, 24, 100, 24, 100]);
    let id2 = SectorId {
        miner: 0,
        number: 0,
    };
    asset_cbor(id2, vec![130, 0, 0]);

    // OnChainSealVerifyInfo
    let verify_info = OnChainSealVerifyInfo {
        sealed_cid: cid.clone(),
        interactive_epoch: 12345678,
        registered_proof: RegisteredProof::StackedDRG2KiBPoSt,
        proof: vec![1, 2, 3, 4, 5, 6, 7, 8],
        deal_id: vec![8, 7, 6],
        sector: 1111,
        seal_rand_epoch: 87654321,
    };
    asset_cbor(
        verify_info.clone(),
        vec![
            135, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48,
            167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199,
            184, 78, 250, 26, 0, 188, 97, 78, 4, 72, 1, 2, 3, 4, 5, 6, 7, 8, 131, 8, 7, 6, 25, 4,
            87, 26, 5, 57, 127, 177,
        ],
    );

    let verify_info2 = OnChainSealVerifyInfo {
        sealed_cid: cid.clone(),
        interactive_epoch: 12345678,
        registered_proof: RegisteredProof::StackedDRG2KiBPoSt,
        proof: vec![1, 2, 3, 4, 5, 6, 7, 8],
        deal_id: vec![],
        sector: 1111,
        seal_rand_epoch: 87654321,
    };
    asset_cbor(
        verify_info2,
        vec![
            135, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48,
            167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199,
            184, 78, 250, 26, 0, 188, 97, 78, 4, 72, 1, 2, 3, 4, 5, 6, 7, 8, 128, 25, 4, 87, 26, 5,
            57, 127, 177,
        ],
    );

    // SealVerifyInfo
    let info = SealVerifyInfo {
        sector: id,
        on_chain: verify_info,
        seal_randomness: [1; 32].into(),
        interactive_randomness: [2; 32].into(),
        unsealed_cid: cid,
    };
    asset_cbor(
        info,
        vec![
            133, 130, 24, 100, 24, 100, 135, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115,
            187, 29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142,
            225, 217, 73, 55, 160, 199, 184, 78, 250, 26, 0, 188, 97, 78, 4, 72, 1, 2, 3, 4, 5, 6,
            7, 8, 131, 8, 7, 6, 25, 4, 87, 26, 5, 57, 127, 177, 88, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 88, 32, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49,
            47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199, 184, 78,
            250,
        ],
    );

    let priv_proof = PrivatePoStCandidateProof {
        proof: RegisteredProof::StackedDRG32GiBSeal,
        externalized: vec![1, 2, 3],
    };
    asset_cbor(priv_proof.clone(), vec![130, 1, 67, 1, 2, 3]);

    let post_can = PoStCandidate {
        proof: RegisteredProof::StackedDRG32GiBSeal,
        partial_ticket: Some([1; 32].into()),
        private_proof: Some(priv_proof),
        sector_id: id,
        challenge_index: 1000,
    };
    asset_cbor(
        post_can.clone(),
        vec![
            133, 1, 88, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 130, 1, 67, 1, 2, 3, 130, 24, 100, 24, 100, 25, 3, 232,
        ],
    );

    let post_can2 = PoStCandidate {
        proof: RegisteredProof::StackedDRG32GiBSeal,
        partial_ticket: None,
        private_proof: None,
        sector_id: id,
        challenge_index: 1000,
    };
    asset_cbor(
        post_can2.clone(),
        vec![133, 1, 64, 130, 0, 64, 130, 24, 100, 24, 100, 25, 3, 232],
    );

    let post_proof = PoStProof {
        proof: RegisteredProof::StackedDRG32GiBSeal,
        proof_bytes: vec![1, 2, 3],
    };
    asset_cbor(post_proof.clone(), vec![130, 1, 67, 1, 2, 3]);

    let verify_info = OnChainPoStVerifyInfo {
        candidates: vec![post_can, post_can2],
        proofs: vec![post_proof],
    };
    asset_cbor(
        verify_info,
        vec![
            130, 130, 133, 1, 88, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 130, 1, 67, 1, 2, 3, 130, 24, 100, 24, 100, 25, 3,
            232, 133, 1, 64, 130, 0, 64, 130, 24, 100, 24, 100, 25, 3, 232, 129, 130, 1, 67, 1, 2,
            3,
        ],
    );
}

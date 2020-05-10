use enum_repr_derive::TryFrom;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use cid::Cid;

use plum_bigint::BigInt;
pub use plum_types::ActorId;
use plum_types::{ChainEpoch, Randomness, SectorNumber, SectorSize};

// The unit of sector weight (power-epochs)
pub type SectorWeight = BigInt;

// The unit of storage power (measured in bytes)
pub type StoragePower = BigInt;

pub type SectorQuality = BigInt;

// The unit of spacetime committed to the network
pub type SpaceTime = BigInt;

pub fn new_storage_power(num: u64) -> StoragePower {
    StoragePower::from(num)
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorId {
    pub miner: ActorId,
    pub number: SectorNumber,
}

#[repr(usize)]
#[derive(TryFrom, Debug, Clone, Copy, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum RegisteredProof {
    StackedDRG32GiBSeal = 1,
    // StackedDRG32GiBPoSt = 2, // No longer used
    StackedDRG2KiBSeal = 3,
    // StackedDRG2KiBPoSt = 4, // No longer used
    StackedDRG8MiBSeal = 5,
    // StackedDRG8MiBPoSt = 6, // No longer used
    StackedDRG512MiBSeal = 7,
    // StackedDRG512MiBPoSt = 8, // No longer used
    StackedDRG2KiBWinningPoSt = 9,
    StackedDRG2KiBWindowPoSt = 10,
    StackedDRG8MiBWinningPoSt = 11,
    StackedDRG8MiBWindowPoSt = 12,
    StackedDRG512MiBWinningPoSt = 13,
    StackedDRG512MiBWindowPoSt = 14,
    StackedDRG32GiBWinningPoSt = 15,
    StackedDRG32GiBWindowPoSt = 16,
    StackedDRG64GiBSeal = 17,
    StackedDRG64GiBWinningPoSt = 18,
    StackedDRG64GiBWindowPoSt = 19,
}

impl RegisteredProof {
    /// `registered_winning_post_proof` produces the PoSt-specific RegisteredProof corresponding
    /// to the receiving RegisteredProof.
    pub fn registered_winning_post_proof(self) -> RegisteredProof {
        match self {
            RegisteredProof::StackedDRG64GiBSeal
            | RegisteredProof::StackedDRG64GiBWindowPoSt
            | RegisteredProof::StackedDRG64GiBWinningPoSt => {
                RegisteredProof::StackedDRG64GiBWinningPoSt
            }
            RegisteredProof::StackedDRG32GiBSeal
            | RegisteredProof::StackedDRG32GiBWindowPoSt
            | RegisteredProof::StackedDRG32GiBWinningPoSt => {
                RegisteredProof::StackedDRG32GiBWinningPoSt
            }
            RegisteredProof::StackedDRG2KiBSeal
            | RegisteredProof::StackedDRG2KiBWindowPoSt
            | RegisteredProof::StackedDRG2KiBWinningPoSt => {
                RegisteredProof::StackedDRG2KiBWinningPoSt
            }
            RegisteredProof::StackedDRG8MiBSeal
            | RegisteredProof::StackedDRG8MiBWindowPoSt
            | RegisteredProof::StackedDRG8MiBWinningPoSt => {
                RegisteredProof::StackedDRG8MiBWinningPoSt
            }
            RegisteredProof::StackedDRG512MiBSeal
            | RegisteredProof::StackedDRG512MiBWindowPoSt
            | RegisteredProof::StackedDRG512MiBWinningPoSt => {
                RegisteredProof::StackedDRG512MiBWinningPoSt
            }
        }
    }

    /// `registered_window_post_proof` produces the PoSt-specific RegisteredProof corresponding
    /// to the receiving RegisteredProof.
    pub fn registered_window_post_proof(self) -> RegisteredProof {
        match self {
            RegisteredProof::StackedDRG64GiBSeal
            | RegisteredProof::StackedDRG64GiBWinningPoSt
            | RegisteredProof::StackedDRG64GiBWindowPoSt => {
                RegisteredProof::StackedDRG64GiBWindowPoSt
            }
            RegisteredProof::StackedDRG32GiBSeal
            | RegisteredProof::StackedDRG32GiBWinningPoSt
            | RegisteredProof::StackedDRG32GiBWindowPoSt => {
                RegisteredProof::StackedDRG32GiBWindowPoSt
            }
            RegisteredProof::StackedDRG2KiBSeal
            | RegisteredProof::StackedDRG2KiBWinningPoSt
            | RegisteredProof::StackedDRG2KiBWindowPoSt => {
                RegisteredProof::StackedDRG2KiBWindowPoSt
            }
            RegisteredProof::StackedDRG8MiBSeal
            | RegisteredProof::StackedDRG8MiBWinningPoSt
            | RegisteredProof::StackedDRG8MiBWindowPoSt => {
                RegisteredProof::StackedDRG8MiBWindowPoSt
            }
            RegisteredProof::StackedDRG512MiBSeal
            | RegisteredProof::StackedDRG512MiBWinningPoSt
            | RegisteredProof::StackedDRG512MiBWindowPoSt => {
                RegisteredProof::StackedDRG512MiBWindowPoSt
            }
        }
    }

    /// `registered_seal_proof` produces the seal-specific RegisteredProof corresponding
    /// to the receiving RegisteredProof.
    pub fn registered_seal_proof(self) -> RegisteredProof {
        match self {
            RegisteredProof::StackedDRG64GiBSeal
            | RegisteredProof::StackedDRG64GiBWindowPoSt
            | RegisteredProof::StackedDRG64GiBWinningPoSt => RegisteredProof::StackedDRG64GiBSeal,
            RegisteredProof::StackedDRG32GiBSeal
            | RegisteredProof::StackedDRG32GiBWindowPoSt
            | RegisteredProof::StackedDRG32GiBWinningPoSt => RegisteredProof::StackedDRG32GiBSeal,
            RegisteredProof::StackedDRG2KiBSeal
            | RegisteredProof::StackedDRG2KiBWindowPoSt
            | RegisteredProof::StackedDRG2KiBWinningPoSt => RegisteredProof::StackedDRG2KiBSeal,
            RegisteredProof::StackedDRG8MiBSeal
            | RegisteredProof::StackedDRG8MiBWindowPoSt
            | RegisteredProof::StackedDRG8MiBWinningPoSt => RegisteredProof::StackedDRG8MiBSeal,
            RegisteredProof::StackedDRG512MiBSeal
            | RegisteredProof::StackedDRG512MiBWindowPoSt
            | RegisteredProof::StackedDRG512MiBWinningPoSt => RegisteredProof::StackedDRG512MiBSeal,
        }
    }

    pub fn sector_size(&self) -> SectorSize {
        let sp = self.registered_seal_proof();
        match sp {
            RegisteredProof::StackedDRG64GiBSeal => 2 * (32 << 30),
            RegisteredProof::StackedDRG32GiBSeal => 32 << 30,
            RegisteredProof::StackedDRG2KiBSeal => 2 << 10,
            RegisteredProof::StackedDRG8MiBSeal => 8 << 20,
            RegisteredProof::StackedDRG512MiBSeal => 512 << 20,
            _ => unreachable!("registered_seal_proof must in above 4 type"),
        }
    }

    pub fn window_post_partition_sectors(&self) -> u64 {
        let sp = self.registered_seal_proof();
        match sp {
            RegisteredProof::StackedDRG64GiBSeal => 2300,
            RegisteredProof::StackedDRG32GiBSeal => 2349,
            RegisteredProof::StackedDRG2KiBSeal
            | RegisteredProof::StackedDRG8MiBSeal
            | RegisteredProof::StackedDRG512MiBSeal => 2,
            _ => unreachable!("registered_seal_proof must in above 4 type"),
        }
    }
}

///
/// Sealing
///

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

///
/// PoSting
///

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct SectorInfo {
    /// RegisteredProof used when sealing - needs to be mapped to PoSt registered proof when used to verify a PoSt
    pub proof: RegisteredProof,
    pub sector_number: SectorNumber,
    /// CommR
    pub sealed_cid: Cid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct PoStProof {
    pub proof: RegisteredProof,
    #[serde(with = "serde_bytes")]
    pub proof_bytes: Vec<u8>,
}

// Information needed to verify a Winning PoSt attached to a block header.
// Note: this is not used within the state machine, but by the consensus/election mechanisms.
#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct WinningPoStVerifyInfo {
    #[serde(with = "plum_hash::h256_cbor")]
    pub randomness: Randomness,
    pub proofs: Vec<PoStProof>,
    pub challenged_sectors: Vec<SectorInfo>,
    pub prover: ActorId, // used to derive 32-byte prover ID
}

// Information needed to verify a Window PoSt submitted directly to a miner actor.
pub type WindowPoStVerifyInfo = WinningPoStVerifyInfo;

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
        registered_proof: RegisteredProof::StackedDRG512MiBSeal,
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
            184, 78, 250, 26, 0, 188, 97, 78, 7, 72, 1, 2, 3, 4, 5, 6, 7, 8, 131, 8, 7, 6, 25, 4,
            87, 26, 5, 57, 127, 177,
        ],
    );

    let verify_info2 = OnChainSealVerifyInfo {
        sealed_cid: cid.clone(),
        interactive_epoch: 12345678,
        registered_proof: RegisteredProof::StackedDRG512MiBSeal,
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
            184, 78, 250, 26, 0, 188, 97, 78, 7, 72, 1, 2, 3, 4, 5, 6, 7, 8, 128, 25, 4, 87, 26, 5,
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
            225, 217, 73, 55, 160, 199, 184, 78, 250, 26, 0, 188, 97, 78, 7, 72, 1, 2, 3, 4, 5, 6,
            7, 8, 131, 8, 7, 6, 25, 4, 87, 26, 5, 57, 127, 177, 88, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 88, 32, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49,
            47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199, 184, 78,
            250,
        ],
    );

    let post_proof = PoStProof {
        proof: RegisteredProof::StackedDRG32GiBSeal,
        proof_bytes: vec![1, 2, 3],
    };
    asset_cbor(post_proof.clone(), vec![130, 1, 67, 1, 2, 3]);
}

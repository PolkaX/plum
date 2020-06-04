// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;
use std::{error, fmt};

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use plum_bigint::BigInt;
use plum_types::ActorId;

/// SectorNumber is a numeric identifier for a sector. It is usually relative to a miner.
pub type SectorNumber = u64;

/// The identifier of a sector.
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorId {
    /// The actor ID of a miner.
    pub miner: ActorId,
    /// The number of the sector.
    pub number: SectorNumber,
}

// Implement CBOR serialization for SectorId.
impl encode::Encode for SectorId {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?.u64(self.miner)?.u64(self.number)?.ok()
    }
}

// Implement CBOR deserialization for SectorId.
impl<'b> decode::Decode<'b> for SectorId {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(SectorId {
            miner: d.u64()?,
            number: d.u64()?,
        })
    }
}

/// The unit of storage power (measured in bytes)
pub type StoragePower = BigInt;

/// The quality of sector.
pub type SectorQuality = BigInt;

/// The unit of spacetime committed to the network
pub type SpaceTime = BigInt;

/// SectorSize indicates one of a set of possible sizes in the network.
/// Ideally, SectorSize would be an enum
///
/// ```
/// #[repr(u64)]
/// pub enum SectorSize {
///   KiB = 1024,
///   MiB = 1_048_576, // 1024^2
///   GiB = 1_073_741_824, //1024^3
///   TiB = 1_099_511_627_776, // 1024^4
///   PiB = 1_125_899_906_842_624, // 1024^5
///   EiB = 1_152_921_504_606_846_976, //1024^6
///   MAX = 18_446_744_073_709_551_615, // 1024^6 * 16 = 2^64 - 1
/// }
/// ```
pub type SectorSize = u64;

/// Abbreviates the size as a human-scale number.
/// This approximates (truncates) the size unless it is a power of 1024.
pub fn readable_sector_size(mut size: SectorSize) -> String {
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut unit = 0;
    while size >= 1024 && unit < UNITS.len() - 1 {
        size /= 1024;
        unit += 1;
    }
    format!("{}{}", size, UNITS[unit])
}

/// This ordering, defines mappings to u64 in a way which MUST never change.
#[doc(hidden)]
#[repr(u64)]
#[derive(
    Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize_repr, Deserialize_repr,
)]
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

impl From<RegisteredProof> for u64 {
    fn from(proof: RegisteredProof) -> Self {
        match proof {
            RegisteredProof::StackedDRG32GiBSeal => 1,
            // RegisteredProof::StackedDRG32GiBPoSt => 2,  // No longer used
            RegisteredProof::StackedDRG2KiBSeal => 3,
            // RegisteredProof::StackedDRG2KiBPoSt => 4,  // No longer used
            RegisteredProof::StackedDRG8MiBSeal => 5,
            // RegisteredProof::StackedDRG8MiBPoSt => 6,  // No longer used
            RegisteredProof::StackedDRG512MiBSeal => 7,
            // RegisteredProof::StackedDRG512MiBPoSt => 8,  // No longer used
            RegisteredProof::StackedDRG2KiBWinningPoSt => 9,
            RegisteredProof::StackedDRG2KiBWindowPoSt => 10,
            RegisteredProof::StackedDRG8MiBWinningPoSt => 11,
            RegisteredProof::StackedDRG8MiBWindowPoSt => 12,
            RegisteredProof::StackedDRG512MiBWinningPoSt => 13,
            RegisteredProof::StackedDRG512MiBWindowPoSt => 14,
            RegisteredProof::StackedDRG32GiBWinningPoSt => 15,
            RegisteredProof::StackedDRG32GiBWindowPoSt => 16,
            RegisteredProof::StackedDRG64GiBSeal => 17,
            RegisteredProof::StackedDRG64GiBWinningPoSt => 18,
            RegisteredProof::StackedDRG64GiBWindowPoSt => 19,
        }
    }
}

impl TryFrom<u64> for RegisteredProof {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => RegisteredProof::StackedDRG32GiBSeal,
            // 2 => RegisteredProof::StackedDRG32GiBPoSt,
            3 => RegisteredProof::StackedDRG2KiBSeal,
            // 4 => RegisteredProof::StackedDRG2KiBPoSt,
            5 => RegisteredProof::StackedDRG8MiBSeal,
            // 6 => RegisteredProof::StackedDRG8MiBPoSt,
            7 => RegisteredProof::StackedDRG512MiBSeal,
            // 8 => RegisteredProof::StackedDRG512MiBPoSt,
            9 => RegisteredProof::StackedDRG2KiBWinningPoSt,
            10 => RegisteredProof::StackedDRG2KiBWindowPoSt,
            11 => RegisteredProof::StackedDRG8MiBWinningPoSt,
            12 => RegisteredProof::StackedDRG8MiBWindowPoSt,
            13 => RegisteredProof::StackedDRG512MiBWinningPoSt,
            14 => RegisteredProof::StackedDRG512MiBWindowPoSt,
            15 => RegisteredProof::StackedDRG32GiBWinningPoSt,
            16 => RegisteredProof::StackedDRG32GiBWindowPoSt,
            17 => RegisteredProof::StackedDRG64GiBSeal,
            18 => RegisteredProof::StackedDRG64GiBWinningPoSt,
            19 => RegisteredProof::StackedDRG64GiBWindowPoSt,
            _ => return Err("unexpected registered proof"),
        })
    }
}

const TWO_KB: SectorSize = 2 << 10;
const EIGHT_MB: SectorSize = 8 << 20;
const FIVE_ONE_TWO_MB: SectorSize = 512 << 20;
const THIRD_TWO_GB: SectorSize = 32 << 30;
const SIXTY_TWO_GB: SectorSize = 2 * (THIRD_TWO_GB);

/// `SectorSize` to `RegisteredProof` meet unknown sector size
#[derive(Debug, Clone)]
pub struct UnknownSectorSizeErr(SectorSize);

impl fmt::Display for UnknownSectorSizeErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown sector size:{:}", self.0)
    }
}

impl error::Error for UnknownSectorSizeErr {}

impl RegisteredProof {
    /// return the sector size of proof
    pub fn sector_size(self) -> SectorSize {
        // Resolve to seal proof and then compute size from that.
        let seal_proof = self.registered_seal_proof();
        match seal_proof {
            RegisteredProof::StackedDRG64GiBSeal => SIXTY_TWO_GB,
            RegisteredProof::StackedDRG32GiBSeal => THIRD_TWO_GB,
            RegisteredProof::StackedDRG2KiBSeal => TWO_KB,
            RegisteredProof::StackedDRG8MiBSeal => EIGHT_MB,
            RegisteredProof::StackedDRG512MiBSeal => FIVE_ONE_TWO_MB,
            _ => unreachable!("registered_seal_proof must in above 4 types"),
        }
    }

    ///
    pub fn from_sector_size(ssize: SectorSize) -> Result<Self, UnknownSectorSizeErr> {
        match ssize {
            TWO_KB => Ok(RegisteredProof::StackedDRG2KiBSeal),
            EIGHT_MB => Ok(RegisteredProof::StackedDRG8MiBSeal),
            FIVE_ONE_TWO_MB => Ok(RegisteredProof::StackedDRG512MiBSeal),
            THIRD_TWO_GB => Ok(RegisteredProof::StackedDRG32GiBSeal),
            SIXTY_TWO_GB => Ok(RegisteredProof::StackedDRG64GiBSeal),
            _ => Err(UnknownSectorSizeErr(ssize)),
        }
    }

    /// Returns the partition size, in sectors, associated with a proof type.
    /// The partition size is the number of sectors proven in a single PoSt proof.
    pub fn window_post_partition_sectors(self) -> u64 {
        // Resolve to seal proof and then compute size from that.
        let seal_proof = self.registered_seal_proof();
        match seal_proof {
            // These numbers must match those used by the proofs library.
            // See https://github.com/filecoin-project/rust-fil-proofs/blob/master/filecoin-proofs/src/constants.rs#L85
            RegisteredProof::StackedDRG64GiBSeal => 2300,
            RegisteredProof::StackedDRG32GiBSeal => 2349,
            RegisteredProof::StackedDRG2KiBSeal
            | RegisteredProof::StackedDRG8MiBSeal
            | RegisteredProof::StackedDRG512MiBSeal => 2,
            _ => unreachable!("registered_seal_proof must in above 4 types"),
        }
    }

    /// RegisteredWinningPoStProof produces the PoSt-specific RegisteredProof
    /// corresponding to the receiving RegisteredProof.
    pub fn registered_winning_post_proof(self) -> Self {
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

    /// RegisteredWindowPoStProof produces the PoSt-specific RegisteredProof
    /// corresponding to the receiving RegisteredProof.
    pub fn registered_window_post_proof(self) -> Self {
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

    /// RegisteredSealProof produces the seal-specific RegisteredProof
    /// corresponding to the receiving RegisteredProof.
    pub fn registered_seal_proof(self) -> Self {
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
}

// Implement CBOR serialization for RegisteredProof.
impl encode::Encode for RegisteredProof {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.u64(u64::from(*self))?.ok()
    }
}

// Implement CBOR deserialization for RegisteredProof.
impl<'b> decode::Decode<'b> for RegisteredProof {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let proof = d.u64()?;
        Ok(RegisteredProof::try_from(proof)
            .map_err(|e| decode::Error::TypeMismatch(proof as u8, e))?)
    }
}

/// Information about a sector necessary for PoSt verification.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorInfo {
    /// RegisteredProof used when sealing - needs to be mapped to PoSt registered proof when used to verify a PoSt
    pub registered_proof: RegisteredProof,
    pub sector_number: SectorNumber,
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
}

// Implement CBOR serialization for SectorInfo.
impl encode::Encode for SectorInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.registered_proof)?
            .u64(self.sector_number)?
            .encode(&self.sealed_cid)?
            .ok()
    }
}

// Implement CBOR deserialization for SectorInfo.
impl<'b> decode::Decode<'b> for SectorInfo {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(SectorInfo {
            registered_proof: d.decode::<RegisteredProof>()?,
            sector_number: d.u64()?,
            sealed_cid: d.decode::<Cid>()?,
        })
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;
use std::{error, fmt};

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use plum_bigint::BigInt;
use plum_types::{ActorId, ChainEpoch};

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

const _2_KB: SectorSize = 2 << 10;
const _8_MB: SectorSize = 8 << 20;
const _512_MB: SectorSize = 512 << 20;
const _32_GB: SectorSize = 32 << 30;
const _64_GB: SectorSize = 2 * (_32_GB);

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

/// `SectorSize` to `RegisteredSealProof` meet unknown sector size
#[derive(Debug, Clone)]
pub struct UnknownSectorSizeErr(SectorSize);

impl fmt::Display for UnknownSectorSizeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown sector size:{:}", self.0)
    }
}

impl error::Error for UnknownSectorSizeErr {}

// This ordering, defines mappings to UInt in a way which MUST never change.
/// define `RegisteredSealProof` same as `ffi::RegisteredSealProof` in filecoin-proofs-api
/// we use our local type for isolate bounds for `filecoin-proofs-api` to reduce influence.
/// And other hand, this type provide cbor encode/decode
#[doc(hidden)]
#[repr(u64)]
#[derive(
    Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum RegisteredSealProof {
    StackedDrg2KiBV1 = 0,
    StackedDrg8MiBV1 = 1,
    StackedDrg512MiBV1 = 2,
    StackedDrg32GiBV1 = 3,
    StackedDrg64GiBV1 = 4,
}

impl From<RegisteredSealProof> for u64 {
    fn from(proof: RegisteredSealProof) -> Self {
        match proof {
            RegisteredSealProof::StackedDrg2KiBV1 => 0,
            RegisteredSealProof::StackedDrg8MiBV1 => 1,
            RegisteredSealProof::StackedDrg512MiBV1 => 2,
            RegisteredSealProof::StackedDrg32GiBV1 => 3,
            RegisteredSealProof::StackedDrg64GiBV1 => 4,
        }
    }
}

impl TryFrom<u64> for RegisteredSealProof {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => RegisteredSealProof::StackedDrg2KiBV1,
            1 => RegisteredSealProof::StackedDrg8MiBV1,
            2 => RegisteredSealProof::StackedDrg512MiBV1,
            3 => RegisteredSealProof::StackedDrg32GiBV1,
            4 => RegisteredSealProof::StackedDrg64GiBV1,
            _ => return Err("unexpected registered seal proof"),
        })
    }
}

/// Implement CBOR serialization for RegisteredSealProof.
impl encode::Encode for RegisteredSealProof {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.u64(u64::from(*self))?.ok()
    }
}

/// Implement CBOR deserialization for RegisteredSealProof.
impl<'b> decode::Decode<'b> for RegisteredSealProof {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let proof = d.u64()?;
        Ok(RegisteredSealProof::try_from(proof)
            .map_err(|e| decode::Error::TypeMismatch(proof as u8, e))?)
    }
}

impl RegisteredSealProof {
    /// Return the sector size of Seal Proof.
    pub fn sector_size(self) -> SectorSize {
        match self {
            RegisteredSealProof::StackedDrg2KiBV1 => _2_KB,
            RegisteredSealProof::StackedDrg8MiBV1 => _8_MB,
            RegisteredSealProof::StackedDrg512MiBV1 => _512_MB,
            RegisteredSealProof::StackedDrg32GiBV1 => _32_GB,
            RegisteredSealProof::StackedDrg64GiBV1 => _64_GB,
        }
    }

    /// Create SealProof from a number(Should be SectorSize).
    /// If the number is not a valid SectorSize, it would return an error.
    pub fn from_sector_size(ssize: SectorSize) -> Result<Self, UnknownSectorSizeErr> {
        match ssize {
            _2_KB => Ok(RegisteredSealProof::StackedDrg2KiBV1),
            _8_MB => Ok(RegisteredSealProof::StackedDrg8MiBV1),
            _512_MB => Ok(RegisteredSealProof::StackedDrg512MiBV1),
            _32_GB => Ok(RegisteredSealProof::StackedDrg32GiBV1),
            _64_GB => Ok(RegisteredSealProof::StackedDrg64GiBV1),
            _ => Err(UnknownSectorSizeErr(ssize)),
        }
    }

    /// Returns the partition size, in sectors, associated with a proof type.
    /// The partition size is the number of sectors proven in a single PoSt proof.
    pub fn window_post_partition_sectors(self) -> u64 {
        match self {
            // These numbers must match those used by the proofs library.
            // See https://github.com/filecoin-project/rust-fil-proofs/blob/master/filecoin-proofs/src/constants.rs#L85
            RegisteredSealProof::StackedDrg64GiBV1 => 2300,
            RegisteredSealProof::StackedDrg32GiBV1 => 2349,
            RegisteredSealProof::StackedDrg2KiBV1
            | RegisteredSealProof::StackedDrg8MiBV1
            | RegisteredSealProof::StackedDrg512MiBV1 => 2,
        }
    }

    /// Return the PoSt-specific RegisteredSealProof corresponding to the receiving RegisteredSealProof.
    pub fn registered_winning_post_proof(self) -> RegisteredPoStProof {
        match self {
            RegisteredSealProof::StackedDrg2KiBV1 => RegisteredPoStProof::StackedDrgWinning2KiBV1,
            RegisteredSealProof::StackedDrg8MiBV1 => RegisteredPoStProof::StackedDrgWinning8MiBV1,
            RegisteredSealProof::StackedDrg512MiBV1 => {
                RegisteredPoStProof::StackedDrgWinning512MiBV1
            }
            RegisteredSealProof::StackedDrg32GiBV1 => RegisteredPoStProof::StackedDrgWinning32GiBV1,
            RegisteredSealProof::StackedDrg64GiBV1 => RegisteredPoStProof::StackedDrgWinning64GiBV1,
        }
    }

    /// Return the PoSt-specific RegisteredSealProof corresponding to the receiving RegisteredSealProof.
    pub fn registered_window_post_proof(self) -> RegisteredPoStProof {
        match self {
            RegisteredSealProof::StackedDrg2KiBV1 => RegisteredPoStProof::StackedDrgWindow2KiBV1,
            RegisteredSealProof::StackedDrg8MiBV1 => RegisteredPoStProof::StackedDrgWindow8MiBV1,
            RegisteredSealProof::StackedDrg512MiBV1 => {
                RegisteredPoStProof::StackedDrgWindow512MiBV1
            }
            RegisteredSealProof::StackedDrg32GiBV1 => RegisteredPoStProof::StackedDrgWindow32GiBV1,
            RegisteredSealProof::StackedDrg64GiBV1 => RegisteredPoStProof::StackedDrgWindow64GiBV1,
        }
    }

    /// Return the maximum duration a sector sealed with this proof may exist between activation and expiration.
    pub const fn sector_maximum_lifetime(self) -> ChainEpoch {
        // For all Stacked DRG sectors, the max is 5 years
        const EPOCHS_PER_YEAR: ChainEpoch = 1_262_277;
        5 * EPOCHS_PER_YEAR
    }
}

/// define `StackedDrgWinning2KiBV1` same as `ffi::StackedDrgWinning2KiBV1` in filecoin-proofs-api
/// we use our local type for isolate bounds for `filecoin-proofs-api` to reduce influence.
/// And other hand, this type provide cbor encode/decode
#[doc(hidden)]
#[repr(u64)]
#[derive(
    Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum RegisteredPoStProof {
    StackedDrgWinning2KiBV1 = 0,
    StackedDrgWinning8MiBV1 = 1,
    StackedDrgWinning512MiBV1 = 2,
    StackedDrgWinning32GiBV1 = 3,
    StackedDrgWinning64GiBV1 = 4,
    StackedDrgWindow2KiBV1 = 5,
    StackedDrgWindow8MiBV1 = 6,
    StackedDrgWindow512MiBV1 = 7,
    StackedDrgWindow32GiBV1 = 8,
    StackedDrgWindow64GiBV1 = 9,
}

impl From<RegisteredPoStProof> for u64 {
    fn from(proof: RegisteredPoStProof) -> Self {
        match proof {
            RegisteredPoStProof::StackedDrgWinning2KiBV1 => 0,
            RegisteredPoStProof::StackedDrgWinning8MiBV1 => 1,
            RegisteredPoStProof::StackedDrgWinning512MiBV1 => 2,
            RegisteredPoStProof::StackedDrgWinning32GiBV1 => 3,
            RegisteredPoStProof::StackedDrgWinning64GiBV1 => 4,
            RegisteredPoStProof::StackedDrgWindow2KiBV1 => 5,
            RegisteredPoStProof::StackedDrgWindow8MiBV1 => 6,
            RegisteredPoStProof::StackedDrgWindow512MiBV1 => 7,
            RegisteredPoStProof::StackedDrgWindow32GiBV1 => 8,
            RegisteredPoStProof::StackedDrgWindow64GiBV1 => 9,
        }
    }
}

impl TryFrom<u64> for RegisteredPoStProof {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => RegisteredPoStProof::StackedDrgWinning2KiBV1,
            1 => RegisteredPoStProof::StackedDrgWinning8MiBV1,
            2 => RegisteredPoStProof::StackedDrgWinning512MiBV1,
            3 => RegisteredPoStProof::StackedDrgWinning32GiBV1,
            4 => RegisteredPoStProof::StackedDrgWinning64GiBV1,
            5 => RegisteredPoStProof::StackedDrgWindow2KiBV1,
            6 => RegisteredPoStProof::StackedDrgWindow8MiBV1,
            7 => RegisteredPoStProof::StackedDrgWindow512MiBV1,
            8 => RegisteredPoStProof::StackedDrgWindow32GiBV1,
            9 => RegisteredPoStProof::StackedDrgWindow64GiBV1,
            _ => return Err("unexpected registered post proof"),
        })
    }
}

/// Implement CBOR serialization for RegisteredPoStProof.
impl encode::Encode for RegisteredPoStProof {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.u64(u64::from(*self))?.ok()
    }
}

/// Implement CBOR deserialization for RegisteredPoStProof.
impl<'b> decode::Decode<'b> for RegisteredPoStProof {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let proof = d.u64()?;
        Ok(RegisteredPoStProof::try_from(proof)
            .map_err(|e| decode::Error::TypeMismatch(proof as u8, e))?)
    }
}

impl RegisteredPoStProof {
    /// Convert PostProof to SealProof
    pub fn registered_seal_proof(self) -> RegisteredSealProof {
        match self {
            RegisteredPoStProof::StackedDrgWinning2KiBV1
            | RegisteredPoStProof::StackedDrgWindow2KiBV1 => RegisteredSealProof::StackedDrg2KiBV1,
            RegisteredPoStProof::StackedDrgWinning8MiBV1
            | RegisteredPoStProof::StackedDrgWindow8MiBV1 => RegisteredSealProof::StackedDrg8MiBV1,
            RegisteredPoStProof::StackedDrgWinning512MiBV1
            | RegisteredPoStProof::StackedDrgWindow512MiBV1 => {
                RegisteredSealProof::StackedDrg512MiBV1
            }
            RegisteredPoStProof::StackedDrgWinning32GiBV1
            | RegisteredPoStProof::StackedDrgWindow32GiBV1 => {
                RegisteredSealProof::StackedDrg32GiBV1
            }
            RegisteredPoStProof::StackedDrgWinning64GiBV1
            | RegisteredPoStProof::StackedDrgWindow64GiBV1 => {
                RegisteredSealProof::StackedDrg64GiBV1
            }
        }
    }

    /// Return Sector size for PostProof
    pub fn sector_size(self) -> SectorSize {
        self.registered_seal_proof().sector_size()
    }

    /// Returns the partition size, in sectors, associated with a proof type.
    /// The partition size is the number of sectors proven in a single PoSt proof.
    pub fn window_post_partition_sectors(self) -> u64 {
        self.registered_seal_proof().window_post_partition_sectors()
    }
}

/// Information about a sector necessary for PoSt verification.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorInfo {
    pub seal_proof: RegisteredSealProof,
    pub sector_number: SectorNumber,
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
}

// Implement CBOR serialization for SectorInfo.
impl encode::Encode for SectorInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.seal_proof)?
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
            seal_proof: d.decode::<RegisteredSealProof>()?,
            sector_number: d.u64()?,
            sealed_cid: d.decode::<Cid>()?,
        })
    }
}

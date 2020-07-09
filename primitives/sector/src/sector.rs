// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::{error, fmt};

use cid::Cid;
use enumn::N;
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

// This ordering, defines mappings to UInt in a way which MUST never change.
/// define `RegisteredSealProof` same as `ffi::RegisteredSealProof` in filecoin-proofs-api
/// we use our local type for isolate bounds for `filecoin-proofs-api` to reduce influence.
/// And other hand, this type provide cbor encode/decode
#[doc(hidden)]
#[repr(i64)]
#[derive(
    Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize_repr, Deserialize_repr, N,
)]
pub enum RegisteredSealProof {
    StackedDrg2KiBV1 = 0,
    StackedDrg8MiBV1 = 1,
    StackedDrg512MiBV1 = 2,
    StackedDrg32GiBV1 = 3,
    StackedDrg64GiBV1 = 4,
}

/// define `StackedDrgWinning2KiBV1` same as `ffi::StackedDrgWinning2KiBV1` in filecoin-proofs-api
/// we use our local type for isolate bounds for `filecoin-proofs-api` to reduce influence.
/// And other hand, this type provide cbor encode/decode
#[doc(hidden)]
#[repr(i64)]
#[derive(
    Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize_repr, Deserialize_repr, N,
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

macro_rules! impl_cbor {
    ($($ProofName:tt),+) => {
$(
/// Implement CBOR serialization for $ProofName.
impl encode::Encode for $ProofName {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.i64(*self as i64)?.ok()
    }
}

/// Implement CBOR deserialization for $ProofName.
impl<'b> decode::Decode<'b> for $ProofName {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let proof = d.i64()?;
        $ProofName::n(proof).ok_or(decode::Error::TypeMismatch(proof as u8, concat!("unexpected ", stringify!($ProofName))))
    }
}
)+
    };
}
impl_cbor!(RegisteredSealProof, RegisteredPoStProof);

impl RegisteredPoStProof {
    /// convert PostProof to SealProof
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
    pub fn sector_size(&self) -> SectorSize {
        self.registered_seal_proof().sector_size()
    }

    /// Returns the partition size, in sectors, associated with a proof type.
    /// The partition size is the number of sectors proven in a single PoSt proof.
    pub fn window_post_partition_sectors(&self) -> u64 {
        self.registered_seal_proof().window_post_partition_sectors()
    }
}

const TWO_KB: SectorSize = 2 << 10;
const EIGHT_MB: SectorSize = 8 << 20;
const FIVE_ONE_TWO_MB: SectorSize = 512 << 20;
const THIRD_TWO_GB: SectorSize = 32 << 30;
const SIXTY_TWO_GB: SectorSize = 2 * (THIRD_TWO_GB);

/// `SectorSize` to `RegisteredSealProof` meet unknown sector size
#[derive(Debug, Clone)]
pub struct UnknownSectorSizeErr(SectorSize);
impl fmt::Display for UnknownSectorSizeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown sector size:{:}", self.0)
    }
}

impl error::Error for UnknownSectorSizeErr {}

impl RegisteredSealProof {
    /// Return SectorSize for Seal Proof
    pub fn sector_size(&self) -> SectorSize {
        match self {
            RegisteredSealProof::StackedDrg2KiBV1 => TWO_KB,
            RegisteredSealProof::StackedDrg8MiBV1 => EIGHT_MB,
            RegisteredSealProof::StackedDrg512MiBV1 => FIVE_ONE_TWO_MB,
            RegisteredSealProof::StackedDrg32GiBV1 => THIRD_TWO_GB,
            RegisteredSealProof::StackedDrg64GiBV1 => SIXTY_TWO_GB,
        }
    }
    /// Create SealProof from a number(Should be SectorSize), if not a valid SectorSize,
    /// would return an error
    pub fn from_sector_size(ssize: SectorSize) -> Result<Self, UnknownSectorSizeErr> {
        match ssize {
            TWO_KB => Ok(RegisteredSealProof::StackedDrg2KiBV1),
            EIGHT_MB => Ok(RegisteredSealProof::StackedDrg8MiBV1),
            FIVE_ONE_TWO_MB => Ok(RegisteredSealProof::StackedDrg512MiBV1),
            THIRD_TWO_GB => Ok(RegisteredSealProof::StackedDrg32GiBV1),
            SIXTY_TWO_GB => Ok(RegisteredSealProof::StackedDrg64GiBV1),
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

    /// RegisteredWinningPoStProof produces the PoSt-specific RegisteredSealProof
    /// corresponding to the receiving RegisteredSealProof.
    pub fn registered_winning_post_proof(&self) -> RegisteredPoStProof {
        match self {
            RegisteredSealProof::StackedDrg64GiBV1 => RegisteredPoStProof::StackedDrgWinning64GiBV1,
            RegisteredSealProof::StackedDrg32GiBV1 => RegisteredPoStProof::StackedDrgWinning32GiBV1,
            RegisteredSealProof::StackedDrg2KiBV1 => RegisteredPoStProof::StackedDrgWinning2KiBV1,
            RegisteredSealProof::StackedDrg8MiBV1 => RegisteredPoStProof::StackedDrgWinning8MiBV1,
            RegisteredSealProof::StackedDrg512MiBV1 => {
                RegisteredPoStProof::StackedDrgWinning512MiBV1
            }
        }
    }
    /// RegisteredWindowPoStProof produces the PoSt-specific RegisteredSealProof
    /// corresponding to the receiving RegisteredSealProof.
    pub fn registered_window_post_proof(self) -> RegisteredPoStProof {
        match self {
            RegisteredSealProof::StackedDrg64GiBV1 => RegisteredPoStProof::StackedDrgWindow64GiBV1,
            RegisteredSealProof::StackedDrg32GiBV1 => RegisteredPoStProof::StackedDrgWindow32GiBV1,
            RegisteredSealProof::StackedDrg2KiBV1 => RegisteredPoStProof::StackedDrgWindow2KiBV1,
            RegisteredSealProof::StackedDrg8MiBV1 => RegisteredPoStProof::StackedDrgWindow8MiBV1,
            RegisteredSealProof::StackedDrg512MiBV1 => {
                RegisteredPoStProof::StackedDrgWindow512MiBV1
            }
        }
    }
    /// SectorMaximumLifetime is the maximum duration a sector sealed with this proof may exist between activation and expiration
    pub fn sector_maximum_lifetime(&self) -> ChainEpoch {
        // For all Stacked DRG sectors, the max is 5 years
        const EPOCHS_PER_YEAR: ChainEpoch = 1_262_277;
        const FIVE_YEARS: ChainEpoch = 5 * EPOCHS_PER_YEAR;
        FIVE_YEARS
    }
}

/// Information about a sector necessary for PoSt verification.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectorInfo {
    /// RegisteredProof used when sealing - needs to be mapped to PoSt registered proof when used to verify a PoSt
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

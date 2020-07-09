// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_types::{ActorId, Randomness};

use crate::sector::{RegisteredPoStProof, SectorInfo};

/// The PoSt proof.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PoStProof {
    #[serde(rename = "PoStProof")]
    pub post_proof: RegisteredPoStProof,
    #[serde(with = "plum_bytes")]
    pub proof_bytes: Vec<u8>,
}

// Implement CBOR serialization for PoStProof.
impl encode::Encode for PoStProof {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?
            .encode(&self.post_proof)?
            .bytes(&self.proof_bytes)?
            .ok()
    }
}

// Implement CBOR deserialization for PoStProof.
impl<'b> decode::Decode<'b> for PoStProof {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(PoStProof {
            post_proof: d.decode::<RegisteredPoStProof>()?,
            proof_bytes: d.bytes()?.to_vec(),
        })
    }
}

// Information needed to verify a Winning PoSt attached to a block header.
// Note: this is not used within the state machine, but by the consensus/election mechanisms.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WinningPoStVerifyInfo {
    pub randomness: Randomness,
    pub proofs: Vec<PoStProof>,
    pub challenged_sectors: Vec<SectorInfo>,
    pub prover: ActorId, // used to derive 32-byte prover ID
}

// Implement CBOR serialization for WinningPoStVerifyInfo.
impl encode::Encode for WinningPoStVerifyInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(4)?
            .encode(&self.randomness)?
            .encode(&self.proofs)?
            .encode(&self.challenged_sectors)?
            .u64(self.prover)?
            .ok()
    }
}

// Implement CBOR deserialization for WinningPoStVerifyInfo.
impl<'b> decode::Decode<'b> for WinningPoStVerifyInfo {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(4));
        Ok(WinningPoStVerifyInfo {
            randomness: d.decode::<Randomness>()?,
            proofs: d.decode::<Vec<PoStProof>>()?,
            challenged_sectors: d.decode::<Vec<SectorInfo>>()?,
            prover: d.u64()?,
        })
    }
}

// Information needed to verify a Window PoSt submitted directly to a miner actor.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WindowPoStVerifyInfo {
    pub randomness: Randomness,
    pub proofs: Vec<PoStProof>,
    pub challenged_sectors: Vec<SectorInfo>,
    pub prover: ActorId, // used to derive 32-byte prover ID
}

// Implement CBOR serialization for WindowPoStVerifyInfo.
impl encode::Encode for WindowPoStVerifyInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(4)?
            .encode(&self.randomness)?
            .encode(&self.proofs)?
            .encode(&self.challenged_sectors)?
            .u64(self.prover)?
            .ok()
    }
}

// Implement CBOR deserialization for WindowPoStVerifyInfo.
impl<'b> decode::Decode<'b> for WindowPoStVerifyInfo {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(4));
        Ok(WindowPoStVerifyInfo {
            randomness: d.decode::<Randomness>()?,
            proofs: d.decode::<Vec<PoStProof>>()?,
            challenged_sectors: d.decode::<Vec<SectorInfo>>()?,
            prover: d.u64()?,
        })
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_types::{DealId, Randomness};

use crate::sector::{RegisteredSealProof, SectorId};

/// Information needed to verify a seal proof.
#[doc(hidden)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealVerifyInfo {
    pub seal_proof: RegisteredSealProof,
    #[serde(flatten)]
    pub sector_id: SectorId,
    #[serde(rename = "DealIDs")]
    pub deal_ids: Vec<DealId>,
    #[serde(with = "plum_bytes")]
    pub randomness: Randomness,
    #[serde(with = "plum_bytes")]
    pub interactive_randomness: Randomness,
    pub proof: Vec<u8>,
    #[serde(rename = "SealedCID")]
    pub sealed_cid: Cid,
    #[serde(rename = "UnsealedCID")]
    pub unsealed_cid: Cid,
}

// Implement CBOR serialization for SealVerifyInfo.
impl encode::Encode for SealVerifyInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(8)?
            .encode(&self.seal_proof)?
            .encode(&self.sector_id)?
            .encode(&self.deal_ids)?
            .encode(&self.randomness)?
            .encode(&self.interactive_randomness)?
            .bytes(&self.proof)?
            .encode(&self.sealed_cid)?
            .encode(&self.unsealed_cid)?
            .ok()
    }
}

// Implement CBOR deserialization for SealVerifyInfo.
impl<'b> decode::Decode<'b> for SealVerifyInfo {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(8));
        Ok(SealVerifyInfo {
            seal_proof: d.decode::<RegisteredSealProof>()?,
            sector_id: d.decode::<SectorId>()?,
            deal_ids: d.decode::<Vec<DealId>>()?,
            randomness: d.decode::<Randomness>()?,
            interactive_randomness: d.decode::<Randomness>()?,
            proof: d.bytes()?.to_vec(),
            sealed_cid: d.decode::<Cid>()?,
            unsealed_cid: d.decode::<Cid>()?,
        })
    }
}

// ///
// #[doc(hidden)]
// #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct OnChainSealVerifyInfo {
//     #[serde(rename = "SealedCID")]
//     pub sealed_cid: Cid,
//     pub interactive_epoch: ChainEpoch,
//     pub registered_proof: RegisteredSealProof,
//     #[serde(with = "plum_bytes")]
//     pub proof: Vec<u8>,
//     #[serde(rename = "DealIDs")]
//     pub deal_ids: Vec<DealId>,
//     pub sector_number: SectorNumber,
//     pub seal_rand_epoch: ChainEpoch, // Used to tie the seal to a chain.
// }

// // Implement CBOR serialization for OnChainSealVerifyInfo.
// impl encode::Encode for OnChainSealVerifyInfo {
//     fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
//         e.array(7)?
//             .encode(&self.sealed_cid)?
//             .i64(self.interactive_epoch)?
//             .encode(&self.registered_proof)?
//             .bytes(&self.proof)?
//             .encode(&self.deal_ids)?
//             .u64(self.sector_number)?
//             .i64(self.seal_rand_epoch)?
//             .ok()
//     }
// }
//
// // Implement CBOR deserialization for OnChainSealVerifyInfo.
// impl<'b> decode::Decode<'b> for OnChainSealVerifyInfo {
//     fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
//         let array_len = d.array()?;
//         assert_eq!(array_len, Some(7));
//         Ok(OnChainSealVerifyInfo {
//             sealed_cid: d.decode::<Cid>()?,
//             interactive_epoch: d.i64()?,
//             registered_proof: d.decode::<RegisteredSealProof>()?,
//             proof: d.bytes()?.to_vec(),
//             deal_ids: d.decode::<Vec<DealId>>()?,
//             sector_number: d.u64()?,
//             seal_rand_epoch: d.i64()?,
//         })
//     }
// }

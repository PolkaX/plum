// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

///
#[repr(i64)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum RegisteredProof {
    StackedDRG32GiBSeal = 1,
    StackedDRG32GiBPoSt = 2, // No longer used
    StackedDRG2KiBSeal = 3,  // No longer used
    StackedDRG2KiBPoSt = 4,
    StackedDRG8MiBSeal = 5, // No longer used
    StackedDRG8MiBPoSt = 6,
    StackedDRG512MiBSeal = 7, // No longer used
    StackedDRG512MiBPoSt = 8,

    StackedDRG2KiBWinningPoSt = 9,
    StackedDRG2KiBWindowPoSt = 10,
    StackedDRG8MiBWinningPoSt = 11,
    StackedDRG8MiBWindowPoSt = 12,
    StackedDRG512MiBWinningPoSt = 13,
    StackedDRG512MiBWindowPoSt = 14,
    StackedDRG32GiBWinningPoSt = 15,
    StackedDRG32GiBWindowPoSt = 16,
}

impl From<RegisteredProof> for i64 {
    fn from(proof: RegisteredProof) -> Self {
        match proof {
            RegisteredProof::StackedDRG32GiBSeal => 1,
            RegisteredProof::StackedDRG32GiBPoSt => 2,
            RegisteredProof::StackedDRG2KiBSeal => 3,
            RegisteredProof::StackedDRG2KiBPoSt => 4,
            RegisteredProof::StackedDRG8MiBSeal => 5,
            RegisteredProof::StackedDRG8MiBPoSt => 6,
            RegisteredProof::StackedDRG512MiBSeal => 7,
            RegisteredProof::StackedDRG512MiBPoSt => 8,
            RegisteredProof::StackedDRG2KiBWinningPoSt => 9,
            RegisteredProof::StackedDRG2KiBWindowPoSt => 10,
            RegisteredProof::StackedDRG8MiBWinningPoSt => 11,
            RegisteredProof::StackedDRG8MiBWindowPoSt => 12,
            RegisteredProof::StackedDRG512MiBWinningPoSt => 13,
            RegisteredProof::StackedDRG512MiBWindowPoSt => 14,
            RegisteredProof::StackedDRG32GiBWinningPoSt => 15,
            RegisteredProof::StackedDRG32GiBWindowPoSt => 16,
        }
    }
}

impl TryFrom<i64> for RegisteredProof {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => RegisteredProof::StackedDRG32GiBSeal,
            2 => RegisteredProof::StackedDRG32GiBPoSt,
            3 => RegisteredProof::StackedDRG2KiBSeal,
            4 => RegisteredProof::StackedDRG2KiBPoSt,
            5 => RegisteredProof::StackedDRG8MiBSeal,
            6 => RegisteredProof::StackedDRG8MiBPoSt,
            7 => RegisteredProof::StackedDRG512MiBSeal,
            8 => RegisteredProof::StackedDRG512MiBPoSt,
            9 => RegisteredProof::StackedDRG2KiBWinningPoSt,
            10 => RegisteredProof::StackedDRG2KiBWindowPoSt,
            11 => RegisteredProof::StackedDRG8MiBWinningPoSt,
            12 => RegisteredProof::StackedDRG8MiBWindowPoSt,
            13 => RegisteredProof::StackedDRG512MiBWinningPoSt,
            14 => RegisteredProof::StackedDRG512MiBWindowPoSt,
            15 => RegisteredProof::StackedDRG32GiBWinningPoSt,
            16 => RegisteredProof::StackedDRG32GiBWindowPoSt,
            _ => return Err("unexpected value for RegisteredProof"),
        })
    }
}

///
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PoStProof {
    pub registered_proof: RegisteredProof,
    #[serde(with = "plum_bytes")]
    pub proof_bytes: Vec<u8>,
}

// Implement CBOR serialization for PoStProof.
impl encode::Encode for PoStProof {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?
            .i64(i64::from(self.registered_proof))?
            .bytes(&self.proof_bytes)?
            .ok()
    }
}

// Implement CBOR deserialization for PoStProof.
impl<'b> decode::Decode<'b> for PoStProof {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));

        let registered_proof = d.i64()?;
        Ok(PoStProof {
            registered_proof: RegisteredProof::try_from(registered_proof)
                .map_err(|e| decode::Error::TypeMismatch(registered_proof as u8, e))?,
            proof_bytes: d.bytes()?.to_vec(),
        })
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! This module provides the conversion utilities between cid and piece/data/replica commitments.
//!
//! Ref filecoin-project/go-fil-commcid

use cid::{Cid, Codec};
use multihash::{Code, FilecoinSealedV1, FilecoinUnsealedV1, MultihashRef};

type Commitment = [u8; 32];

///
pub const FILECOIN_CODEC_TYPE: Codec = Codec::Raw;

///
#[doc(hidden)]
#[derive(thiserror::Error, Debug)]
pub enum CommCidErr {
    #[error("invalid multihash code: {0:?}")]
    InvalidHash(Code),
    #[error("this multihash code: {0:?} is unsupported")]
    UnsupportedMultihashCode(Code),
    #[error("receive an unexpect multihash code (expected: {0:?}, found: {1:?})")]
    UnexpectedMultihashCode(Code, Code),
}

/// Converts a raw commitment to a CID given the multihash type.
pub fn commitment_to_cid(commitment: Commitment, code: Code) -> Result<Cid, CommCidErr> {
    let hash = match code {
        multihash::Code::FilecoinUnsealedV1 => FilecoinUnsealedV1::digest(&commitment),
        multihash::Code::FilecoinSealedV1 => FilecoinSealedV1::digest(&commitment),
        _ => return Err(CommCidErr::UnsupportedMultihashCode(code)),
    };
    Ok(Cid::new_v1(FILECOIN_CODEC_TYPE, hash))
}

/// Extracts the raw data commitment from a CID given the multihash type.
pub fn cid_to_commitment(cid: &Cid, multihash_code: Code) -> Result<Commitment, CommCidErr> {
    let hash = cid_to_multihash(cid, multihash_code)?;
    let mut c = Commitment::default();
    // hash.digest must be 32 bytes, if not panic here.
    c.copy_from_slice(hash.digest());
    Ok(c)
}

fn cid_to_multihash(cid: &Cid, expected: Code) -> Result<MultihashRef, CommCidErr> {
    let hash = cid.hash();
    let code = hash.algorithm();
    match code {
        fl_code @ Code::FilecoinSealedV1 | fl_code @ Code::FilecoinUnsealedV1 => {
            if fl_code != expected {
                Err(CommCidErr::UnexpectedMultihashCode(expected, fl_code))
            } else {
                Ok(hash)
            }
        }
        _ => Err(CommCidErr::InvalidHash(code)),
    }
}

/// Converts a raw commitment to a CID with sealed hash type.
pub fn replica_commitment_v1_to_cid(commitment: Commitment) -> Cid {
    commitment_to_cid(commitment, Code::FilecoinSealedV1)
        .expect("`commitment_to_cid` must receive `FcSealedV1`")
}

/// Converts a raw commitment to a CID with unsealed hash type.
pub fn data_commitment_v1_to_cid(commitment: Commitment) -> Cid {
    commitment_to_cid(commitment, Code::FilecoinUnsealedV1)
        .expect("`commitment_to_cid` must receive `FcUnsealedV1`")
}

/// Converts a commP to a CID, equivalent to data_commitment_v1_to_cid().
pub fn piece_commitment_v1_to_cid(commitment: Commitment) -> Cid {
    data_commitment_v1_to_cid(commitment)
}

/// Extracts the raw commiment from a CID that uses sealed hashing function.
pub fn cid_to_replica_commitment_v1(cid: &Cid) -> Result<Commitment, CommCidErr> {
    cid_to_commitment(cid, Code::FilecoinSealedV1)
}

/// Extracts the raw commiment from a CID that uses unsealed hashing function.
pub fn cid_to_data_commitment_v1(cid: &Cid) -> Result<Commitment, CommCidErr> {
    cid_to_commitment(cid, Code::FilecoinUnsealedV1)
}

/// Converts a CID to a commP, equivalent to cid_to_data_commitment_v1()
pub fn cid_to_piece_commitment_v1(cid: &Cid) -> Result<Commitment, CommCidErr> {
    cid_to_data_commitment_v1(cid)
}

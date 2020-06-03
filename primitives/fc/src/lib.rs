// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod comm;

pub use self::comm::{
    cid_to_data_commitment_v1, cid_to_piece_commitment_v1, cid_to_replica_commitment_v1,
    commitment_to_cid, data_commitment_v1_to_cid, piece_commitment_v1_to_cid,
    replica_commitment_v1_to_cid, CommCidErr, FILECOIN_CODEC_TYPE,
};

// use filecoin_proofs::types::{PieceInfo as FcPieceInfo, UnpaddedBytesAmount};

// use plum_piece::PieceInfo;


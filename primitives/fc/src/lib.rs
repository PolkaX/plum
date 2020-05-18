// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod comm;

pub use self::comm::{
    cid_to_data_commitment_v1, cid_to_piece_commitment_v1, cid_to_replica_commitment_v1,
    commitment_to_cid, data_commitment_v1_to_cid, piece_commitment_v1_to_cid,
    replica_commitment_v1_to_cid, CommCidErr, FILECOIN_CODEC_TYPE,
};

use filecoin_proofs::types::{PieceInfo as FcPieceInfo, UnpaddedBytesAmount};

use plum_piece::PieceInfo;

/// Convert piece info to filecoin proof piece info.
pub fn convert_pieceinfo(pieceinfo: PieceInfo) -> Result<FcPieceInfo, CommCidErr> {
    let unpadded = pieceinfo.size.unpadded();
    let commitment = cid_to_piece_commitment_v1(&pieceinfo.piece_cid)?;
    Ok(FcPieceInfo {
        commitment,
        size: UnpaddedBytesAmount(u64::from(unpadded)),
    })
}

/// convert piece info list to filecoin proof pieceinfo list
pub fn convert_pieceinfos(pieceinfos: Vec<PieceInfo>) -> Result<Vec<FcPieceInfo>, CommCidErr> {
    let mut v = Vec::with_capacity(pieceinfos.len());
    for info in pieceinfos {
        let p = convert_pieceinfo(info)?;
        v.push(p);
    }
    Ok(v)
}

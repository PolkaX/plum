// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use crate::size::PaddedPieceSize;

/// The information of a piece.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PieceInfo {
    /// Size in nodes. For BLS12-381 (capacity 254 bits), must be >= 16. (16 * 8 = 128)
    pub size: PaddedPieceSize,
    /// The CID of the piece.
    #[serde(rename = "PieceCID")]
    pub piece_cid: Cid,
}

// Implement CBOR serialization for PieceInfo.
impl encode::Encode for PieceInfo {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?.u64(self.size.0)?.encode(&self.piece_cid)?.ok()
    }
}

// Implement CBOR deserialization for PieceInfo.
impl<'b> decode::Decode<'b> for PieceInfo {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(Self {
            size: PaddedPieceSize(d.u64()?),
            piece_cid: d.decode()?,
        })
    }
}

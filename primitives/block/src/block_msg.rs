// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use crate::header::BlockHeader;

/// The block message.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockMsg {
    /// The block header.
    pub header: BlockHeader,
    /// The CIDs of `BLS` messages.
    pub bls_messages: Vec<Cid>,
    /// The CIDs of `Secp256k1` messages.
    pub secpk_messages: Vec<Cid>,
}

impl BlockMsg {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        self.header.cid()
    }
}

// Implement CBOR serialization for BlockMsg.
impl encode::Encode for BlockMsg {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.header)?
            .encode(&self.bls_messages)?
            .encode(&self.secpk_messages)?
            .ok()
    }
}

// Implement CBOR deserialization for BlockMsg.
impl<'b> decode::Decode<'b> for BlockMsg {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(BlockMsg {
            header: d.decode::<BlockHeader>()?,
            bls_messages: d.decode::<Vec<Cid>>()?,
            secpk_messages: d.decode::<Vec<Cid>>()?,
        })
    }
}

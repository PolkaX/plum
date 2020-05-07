// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_message::{SignedMessage, UnsignedMessage};

use crate::header::BlockHeader;

/// The complete block.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    /// The block header.
    pub header: BlockHeader,
    /// The `BLS` messages.
    pub bls_messages: Vec<UnsignedMessage>,
    /// The `Secp256k1` messages.
    pub secpk_messages: Vec<SignedMessage>,
}

impl Block {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        self.header.cid()
    }
}

// Implement CBOR serialization for Block.
impl encode::Encode for Block {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.header)?
            .encode(&self.bls_messages)?
            .encode(&self.secpk_messages)?
            .ok()
    }
}

// Implement CBOR deserialization for Block.
impl<'b> decode::Decode<'b> for Block {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(Block {
            header: d.decode::<BlockHeader>()?,
            bls_messages: d.decode::<Vec<UnsignedMessage>>()?,
            secpk_messages: d.decode::<Vec<SignedMessage>>()?,
        })
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec};
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

///
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MsgMeta {
    ///
    pub bls_messages: Cid,
    ///
    pub secpk_messages: Cid,
}

impl MsgMeta {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        let data =
            minicbor::to_vec(self).expect("CBOR serialization of MsgMeta shouldn't be failed");
        self.cid_with_data(data)
    }

    /// Convert to the CID with the given CBOR serialized data of MsgData.
    ///
    /// For cases where serialized data of the MsgData is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        let hash = multihash::Blake2b256::digest(data.as_ref());
        Cid::new_v1(Codec::DagCBOR, hash)
    }
}

// Implement CBOR serialization for MsgMeta.
impl encode::Encode for MsgMeta {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?
            .encode(&self.bls_messages)?
            .encode(&self.secpk_messages)?
            .ok()
    }
}

// Implement CBOR deserialization for MsgMeta.
impl<'b> decode::Decode<'b> for MsgMeta {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(MsgMeta {
            bls_messages: d.decode::<Cid>()?,
            secpk_messages: d.decode::<Cid>()?,
        })
    }
}

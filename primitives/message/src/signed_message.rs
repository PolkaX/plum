// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec, IntoExt};
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_crypto::{Signature, SignatureType};

use crate::unsigned_message::UnsignedMessage;

/// The signed message (a message with signature).
#[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignedMessage {
    /// The unsigned message.
    pub message: UnsignedMessage,
    /// The signature.
    pub signature: Signature,
}

impl SignedMessage {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        if self.signature.r#type() == SignatureType::Bls {
            return self.message.cid();
        }
        let data = minicbor::to_vec(self)
            .expect("CBOR serialization of SignedMessage shouldn't be failed");
        let hash = multihash::Blake2b256::digest(&data).into_ext();
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    /// Convert to the CID with the given CBOR serialized data of SignedMessage.
    ///
    /// For cases where serialized data of the SignedMessage is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        if self.signature.r#type() == SignatureType::Bls {
            return self.message.cid();
        }
        let hash = multihash::Blake2b256::digest(data.as_ref()).into_ext();
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    /// Returns the size of cbor encoded SignedMessage.
    pub fn cbor_encoded_len(&self) -> usize {
        let data = minicbor::to_vec(self)
            .expect("CBOR serialization of SignedMessage shouldn't be failed");
        data.len()
    }

    ///
    pub fn vm_message(&self) -> &UnsignedMessage {
        &self.message
    }
}

// Implement CBOR serialization for SignedMessage.
impl encode::Encode for SignedMessage {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?
            .encode(&self.message)?
            .encode(&self.signature)?
            .ok()
    }
}

// Implement CBOR deserialization for SignedMessage.
impl<'b> decode::Decode<'b> for SignedMessage {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(SignedMessage {
            message: d.decode::<UnsignedMessage>()?,
            signature: d.decode::<Signature>()?,
        })
    }
}

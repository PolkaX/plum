// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use fixed_hash::construct_fixed_hash;
use minicbor::{decode, encode, Decoder, Encoder};

construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    pub struct H256(32);
}

// Implement CBOR serialization for H256.
impl encode::Encode for H256 {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(self.as_bytes())?.ok()
    }
}

// Implement CBOR deserialization for H256.
impl<'b> decode::Decode<'b> for H256 {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        if bytes.len() == H256::len_bytes() {
            Ok(H256::from_slice(bytes))
        } else {
            Err(decode::Error::Message("H256 length must be 32 Bytes"))
        }
    }
}

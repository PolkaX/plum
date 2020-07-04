// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

// Implement CBOR serialization for U256.
impl encode::Encode for U256 {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        let mut bytes = [0u8; 4 * 8];
        self.to_big_endian(&mut bytes);
        e.bytes(&bytes)?.ok()
    }
}

// Implement CBOR deserialization for U256.
impl<'b> decode::Decode<'b> for U256 {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        Ok(U256::from_big_endian(bytes))
    }
}

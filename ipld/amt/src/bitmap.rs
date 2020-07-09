// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;

use minicbor::{decode, encode, Decoder, Encoder};

///
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BitMap(u8);

impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ", {:08b}", self.0)
    }
}

impl fmt::Binary for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ", {:08b}", self.0)
    }
}

// Implement CBOR serialization for BitMap.
impl encode::Encode for BitMap {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&[self.0])?.ok()
    }
}

// Implement CBOR deserialization for BitMap.
impl<'b> decode::Decode<'b> for BitMap {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let byte = d.bytes()?.get(0).expect("expected bitmap byte");
        Ok(Self(*byte))
    }
}

impl BitMap {
    /// Check if the bitmap is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if the index of bitmap has bit.
    pub fn has_bit(&self, i: u8) -> bool {
        self.0 & (1 << i) != 0
    }

    /// Set bit at the index of bitmap.
    pub fn set_bit(&mut self, i: u8) {
        self.0 |= 1 << i
    }

    /// Unset bit at the index of bitmap.
    pub fn unset_bit(&mut self, i: u8) {
        self.0 &= !(1 << i)
    }
}

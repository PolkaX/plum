// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! Advanced RLE+ implementation.

#![deny(missing_docs)]

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser};

use rleplus::RlePlus;

///
#[derive(Clone, Debug, Default)]
pub struct BitField {
    /// The underlying RLE+ encoded bitvec.
    rle: RlePlus,
    /// Bits set to 1. Never overlaps with `unset`.
    set: HashSet<u64>,
    /// Bits set to 0. Never overlaps with `set`.
    unset: HashSet<u64>,
}

impl From<RlePlus> for BitField {
    fn from(rle: RlePlus) -> Self {
        Self {
            rle,
            ..Default::default()
        }
    }
}

impl From<BitField> for RlePlus {
    fn from(bitfield: BitField) -> Self {
        if bitfield.set.is_empty() && bitfield.unset.is_empty() {
            bitfield.rle
        } else {
            todo!()
        }
    }
}

impl BitField {
    /// Create an empty bit field
    pub fn new() -> Self {
        Self::default()
    }

    /// Set ...s bit in the bit field.
    /// Add the bit at a given index to the bit field.
    pub fn set(&mut self, bit: u64) {
        self.unset.remove(&bit);
        self.set.insert(bit);
    }

    /// Unset ...s bit in the bit field.
    /// Remove the bit at a given index from the bit field.
    pub fn unset(&mut self, bit: u64) {
        self.set.remove(&bit);
        self.unset.insert(bit);
    }

    /// Return the number of set bits in the bit field.
    pub fn len(&self) -> u64 {
        todo!()
    }

    /// Return `true` if the bit field is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///
    pub fn merge(&self, other: &Self) -> Self {
        todo!()
    }

    ///
    pub fn intersect(&self, other: &Self) -> Self {
        todo!()
    }
}

// Implement CBOR serialization for BitField.
impl encode::Encode for BitField {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        // e.bytes(&rle::encode(self.0.iter()))?.ok()
        todo!()
    }
}

// Implement CBOR deserialization for BitField.
impl<'b> decode::Decode<'b> for BitField {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        // let bytes = d.bytes()?;
        // let set: Vec<u64> =
        //     rle::decode(bytes).map_err(|_| decode::Error::Message("RLE+ decode error"))?;
        // Ok(BitField(set.into_iter().collect()))
        todo!()
    }
}

// Implement JSON serialization for BitField.
impl ser::Serialize for BitField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serde_bytes::serialize(self.rle.as_bytes(), serializer)
    }
}

// Implement JSON serialization for BitField.
impl<'de> de::Deserialize<'de> for BitField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes = serde_bytes::deserialize::<Vec<u8>>(deserializer)?;
        let rle = RlePlus::new(bytes.into()).map_err(de::Error::custom)?;
        Ok(Self {
            rle,
            ..Default::default()
        })
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip_codec(src: &BitField) -> BitField {
        let ser = minicbor::to_vec(src).unwrap();
        let bitfield = minicbor::decode::<BitField>(&ser).unwrap();
        assert_eq!(src, &bitfield);
        bitfield
    }

    #[test]
    fn test_bit_filed_has() {
        let mut bf = BitField::new();
        bf.insert(1);
        bf.insert(2);
        bf.insert(3);
        bf.insert(4);
        bf.insert(5);

        assert!(bf.contains(&1));
        assert!(!bf.contains(&6));

        let bf2 = roundtrip_codec(&bf);
        assert!(bf2.contains(&1));
        assert!(!bf2.contains(&6));
    }

    #[test]
    fn test_codec() {
        let mut bf = BitField::new();
        bf.insert(2);
        bf.insert(7);
        let v = minicbor::to_vec(&bf).unwrap();
        assert_eq!(v, vec![67, 80, 74, 1]);
        let _ = roundtrip_codec(&bf);
    }
}
*/

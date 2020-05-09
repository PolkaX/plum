// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser};

///
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BitField(BTreeSet<u64>);

impl BitField {
    ///
    pub fn new() -> Self {
        BitField(BTreeSet::new())
    }
}

impl AsRef<BTreeSet<u64>> for BitField {
    fn as_ref(&self) -> &BTreeSet<u64> {
        &self.0
    }
}

impl AsMut<BTreeSet<u64>> for BitField {
    fn as_mut(&mut self) -> &mut BTreeSet<u64> {
        &mut self.0
    }
}

impl Borrow<BTreeSet<u64>> for BitField {
    fn borrow(&self) -> &BTreeSet<u64> {
        self.0.borrow()
    }
}

impl BorrowMut<BTreeSet<u64>> for BitField {
    fn borrow_mut(&mut self) -> &mut BTreeSet<u64> {
        self.0.borrow_mut()
    }
}

impl Deref for BitField {
    type Target = BTreeSet<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<u64>> for BitField {
    fn from(v: Vec<u64>) -> Self {
        BitField(v.into_iter().collect())
    }
}

impl From<BTreeSet<u64>> for BitField {
    fn from(v: BTreeSet<u64>) -> Self {
        BitField(v)
    }
}

// Implement CBOR serialization for BitField.
impl encode::Encode for BitField {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&rle::encode(self.0.iter()))?.ok()
    }
}

// Implement CBOR deserialization for BitField.
impl<'b> decode::Decode<'b> for BitField {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        let set: Vec<u64> =
            rle::decode(bytes).map_err(|_| decode::Error::Message("RLE+ decode error"))?;
        Ok(BitField(set.into_iter().collect()))
    }
}

// Implement JSON serialization for BitField.
impl ser::Serialize for BitField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let bytes = rle::encode(self.0.iter());
        serde_bytes::serialize(&bytes, serializer)
    }
}

// Implement JSON serialization for BitField.
impl<'de> de::Deserialize<'de> for BitField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde_bytes::deserialize(deserializer)?;
        let set: Vec<u64> = rle::decode(bytes).map_err(de::Error::custom)?;
        Ok(BitField(set.into_iter().collect()))
    }
}

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

use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};

use codec_rle::{rle_decode, rle_encode};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BitField(BTreeSet<u64>);

impl BitField {
    pub fn new() -> Self {
        BitField(BTreeSet::new())
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

impl Serialize for BitField {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = rle_encode(self.0.iter());
        serde_bytes::serialize(&v, serializer)
    }
}

impl<'de> Deserialize<'de> for BitField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<u8> = serde_bytes::deserialize(deserializer)?;
        let set: Vec<u64> = rle_decode(v).map_err(D::Error::custom)?;
        Ok(BitField(set.into_iter().collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip_codec(src: &BitField) -> BitField {
        let v = serde_cbor::to_vec(src).unwrap();
        let new = serde_cbor::from_slice(&v).unwrap();
        assert_eq!(*src, new);
        new
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
        let v = serde_cbor::to_vec(&bf).unwrap();
        assert_eq!(v, vec![67, 80, 74, 1]);
        let _ = roundtrip_codec(&bf);
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

impl U256 {
    // Returns the index within the collapsed array corresponding to the given bit in the bitset.
    // The collapsed array contains only one entry per bit set in the bitfield,
    // and this function is used to map the indices.
    pub(crate) fn index_for_bit_pos(&self, bit_pos: u8) -> usize {
        let mask = (U256::one() << bit_pos) - U256::one(); // 2^bit_pos - 1
        let result = *self & mask;
        result.count_ones() as usize
    }

    pub(crate) fn count_ones(&self) -> u32 {
        self.0.iter().map(|a| a.count_ones()).sum()
    }

    pub(crate) fn set_bit(&mut self, idx: u8) {
        let elem_idx = (idx / 64) as usize;
        let bit_idx = idx % 64;
        self.0[elem_idx] |= 1 << bit_idx as u64;
    }

    pub(crate) fn unset_bit(&mut self, idx: u8) {
        let elem_idx = (idx / 64) as usize;
        let bit_idx = idx % 64;
        self.0[elem_idx] &= !(1 << bit_idx as u64)
    }

    pub(crate) fn has_bit(&self, index: u8) -> bool {
        self.bit(index as usize)
    }
}

impl std::fmt::Binary for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0;
        write!(f, "{:064b}_{:064b}_{:064b}_{:064b}", value[0], value[1], value[2], value[3])
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitfield() {
        let mut bitfield = U256::zero();
        assert_eq!(bitfield.count_ones(), 0);

        bitfield.set_bit(0);
        assert!(bitfield.has_bit(0));
        bitfield.set_bit(64);
        assert!(bitfield.has_bit(64));
        bitfield.set_bit(128);
        assert!(bitfield.has_bit(128));
        bitfield.set_bit(192);
        assert!(bitfield.has_bit(192));
        bitfield.set_bit(255);
        assert!(bitfield.has_bit(255));
        assert_eq!(bitfield.count_ones(), 5);
        // dbg!(bitfield.index_for_bit_pos(0));    // 0
        // dbg!(bitfield.index_for_bit_pos(1));    // 1
        // dbg!(bitfield.index_for_bit_pos(64));   // 1
        // dbg!(bitfield.index_for_bit_pos(65));   // 2
        // dbg!(bitfield.index_for_bit_pos(128));  // 2
        // dbg!(bitfield.index_for_bit_pos(129));  // 3
        // dbg!(bitfield.index_for_bit_pos(192));  // 3
        // dbg!(bitfield.index_for_bit_pos(193));  // 4
        // dbg!(bitfield.index_for_bit_pos(254));  // 4
        // dbg!(bitfield.index_for_bit_pos(255));  // 4

        bitfield.unset_bit(0);
        assert!(!bitfield.has_bit(0));
        bitfield.unset_bit(64);
        assert!(!bitfield.has_bit(64));
        bitfield.unset_bit(128);
        assert!(!bitfield.has_bit(128));
        bitfield.unset_bit(192);
        assert!(!bitfield.has_bit(192));
        bitfield.unset_bit(255);
        assert!(!bitfield.has_bit(255));
        assert_eq!(bitfield.count_ones(), 0);
    }
}

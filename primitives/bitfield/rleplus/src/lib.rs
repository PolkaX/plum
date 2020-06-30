// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! # RLE+ Bitset Encoding
//!
//! See [filecoin-project specs](https://github.com/filecoin-project/specs/blob/master/src/listings/data_structures.md) for details.
//!
//! RLE+ is a lossless compression format based on [RLE](https://en.wikipedia.org/wiki/Run-length_encoding).
//! Its primary goal is to reduce the size in the case of many individual bits, where RLE breaks down quickly,
//! while keeping the same level of compression for large sets of contiguous bits.
//!
//! In tests it has shown to be more compact than RLE itself, as well as [Concise](https://arxiv.org/pdf/1004.0403.pdf) and [Roaring](https://roaringbitmap.org/).
//!
//! ## Format
//!
//! The format consists of a header, followed by a series of blocks, of which there are three different types.
//!
//! The format can be expressed as the following [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) grammar.
//!
//! ```text
//!     <encoding> ::= <header> <blocks>
//!       <header> ::= <version> <bit>
//!      <version> ::= "00"
//!       <blocks> ::= <block> <blocks> | ""
//!        <block> ::= <block_single> | <block_short> | <block_long>
//! <block_single> ::= "1"
//!  <block_short> ::= "01" <bit> <bit> <bit> <bit>
//!   <block_long> ::= "00" <unsigned_varint>
//!          <bit> ::= "0" | "1"
//! ```
//!
//! An `<unsigned_varint>` is defined as specified [here](https://github.com/multiformats/unsigned-varint).
//!
//! ### Blocks
//!
//! The blocks represent how many bits, of the current bit type there are. As `0` and `1` alternate in a bit vector
//! the initial bit, which is stored in the header, is enough to determine if a length is currently referencing
//! a set of `0`s, or `1`s.
//!
//! #### Block Single
//!
//! If the running length of the current bit is only `1`, it is encoded as a single set bit.
//!
//! #### Block Short
//!
//! If the running length is less than `16`, it can be encoded into up to four bits, which a short block
//! represents. The length is encoded into a 4 bits, and prefixed with `01`, to indicate a short block.
//!
//! #### Block Long
//!
//! If the running length is `16` or larger, it is encoded into a varint, and then prefixed with `00` to indicate
//! a long block.
//!
//! > **Note:** The encoding is unique, so no matter which algorithm for encoding is used, it should produce
//! > the same encoding, given the same input.
//!
//! #### Bit Numbering
//!
//! For Filecoin, byte arrays representing RLE+ bitstreams are encoded using [LSB 0](https://en.wikipedia.org/wiki/Bit_numbering#LSB_0_bit_numbering) bit numbering.
//!

#![deny(missing_docs)]

mod error;
mod iter;
mod decoder;
mod encoder;

pub use self::error::Error;

/// An internal RLE+ encoded bit field.
pub type BitVec = bitvec::vec::BitVec<bitvec::order::Lsb0, u8>;

/// An RLE+ encoded bit field wrapper.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct RlePlus(BitVec);

impl RlePlus {
    /// Create a new `RlePlus` with the encoded bitvec.
    /// Return an error if the given bitvec is not RLE+ encoded correctly.
    pub fn new(encoded: BitVec) -> Result<Self, Error> {
        todo!()
    }

    ///
    pub fn encode(raw: &BitVec) -> Self {
        todo!()
    }

    ///
    pub fn decode(&self) -> BitVec {
        todo!()
    }

    /// Check if the RLE+ encoded data contains the bit at the given `index`.
    pub fn has(&self, index: usize) -> bool {
        todo!()
    }

    /// Return a byte slice of the bit field's content.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Convert the bit field's content into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::RlePlus;

    use bitvec::{bitvec, order::Lsb0, vec::BitVec};

    #[test]
    fn test_rle_plus_roundtrip() {
        let cases = vec![
            (
                bitvec![Lsb0, u8; 1; 8],
                bitvec![Lsb0, u8;
                    0, 0,
                    1,
                    0, 1,
                    0, 0, 0, 1,
                ],
            ),
            (
                bitvec![Lsb0, u8; 1; 25],
                bitvec![Lsb0, u8;
                    0, 0,
                    1,
                    0, 0,1
                    , 0, 0, 1, 1, 0, 0, 0
                ],
            ),
            (
                bitvec![Lsb0, u8; 1, 1, 1, 1, 0, 1, 1, 1],
                bitvec![Lsb0, u8;
                    0, 0,
                    1,
                    0, 1,
                    0, 0, 1, 0,
                    1,
                    0, 1,
                    1, 1, 0, 0,
                ],
            )
        ];

        for (raw, expected) in cases {
            let encoded = RlePlus::encode(&raw);
            assert_eq!(encoded, RlePlus::new(expected.clone()));

            let decoded = RlePlus::new(expected).unwrap().decode();
            assert_eq!(decoded, raw);
        }
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;

#[cfg(not(feature = "identity-hash"))]
pub(crate) fn hash<T: AsRef<[u8]>>(bytes: T) -> [u8; 8] {
    let hash = murmur3::murmur3_x64_128(&mut bytes.as_ref(), 0).unwrap();
    let hash = hash.to_be_bytes();
    let mut result = [0u8; 8];
    result.copy_from_slice(&hash[8..]);
    result
}

#[cfg(feature = "identity-hash")]
pub(crate) fn hash<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut hash = [0u8; 32];
    for (index, byte) in bytes.as_ref().iter().take(32).enumerate() {
        hash[index] = *byte;
    }
    hash
}

/// HashBits is a helper that allows the reading of the 'next n bits' as an integer.
pub(crate) struct HashBits<'a> {
    buf: &'a [u8],
    // consumed bits of buf.
    consumed: usize,
}

impl<'a> HashBits<'a> {
    /// Create a new HashBits with the given buffer.
    pub fn new(buf: &'a [u8]) -> HashBits<'a> {
        Self::new_with_consumed(buf, 0)
    }

    /// Create a new HashBits with the given buffer and consumed bits.
    pub fn new_with_consumed(buf: &'a [u8], consumed: usize) -> HashBits<'a> {
        HashBits { buf, consumed }
    }

    /// Return the next 'i' bits of the HashBits value as an integer,
    /// or an error if there aren't enough bits.
    pub fn next(&mut self, i: usize) -> Option<usize> {
        if self.consumed + i > self.buf.len() * 8 {
            None
        } else {
            Some(self.next_bits(i))
        }
    }

    fn next_bits(&mut self, i: usize) -> usize {
        let cur_byte_index = self.consumed / 8;
        let left_bit = 8 - (self.consumed % 8); // left bit of current byte, <= 8

        let cur_byte = self.buf[cur_byte_index];
        match i.cmp(&left_bit) {
            Ordering::Equal => {
                // return the all left bits of current byte as a integer.
                let out = mkmask(i) & cur_byte;
                self.consumed += i;
                out as usize
            },
            Ordering::Less => {
                // return the n bits that (consumed % 8 <= n < consumed % 8 + i) of current byte as a integer.
                let a = cur_byte & mkmask(left_bit); // mask out the high bits of current byte we don't want
                let b = a & !mkmask(left_bit - i);   // mask out the low bits of current byte we don't want
                let c = b >> (left_bit - i) as u8;       // shift whats left down
                self.consumed += i;
                c as usize
            },
            Ordering::Greater => {
                // return the all left bit of current byte and remaining (i - left_bit) bits as a integer.
                let mut out = (cur_byte & mkmask(left_bit)) as usize; // mask out the high bits of current byte we don't want
                out <<= i - left_bit;
                self.consumed += left_bit;
                out += self.next_bits(i - left_bit);
                out
            },
        }
    }
}

#[inline]
fn mkmask(n: usize) -> u8 {
    ((1 << n) - 1) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "identity-hash"))]
    fn test_hash() {
        let hash = hash(b"abcd");
        assert_eq!(hash, [184, 123, 183, 214, 70, 86, 205, 79]);
    }

    #[test]
    #[cfg(feature = "identity-hash")]
    fn test_hash() {
        let hash = hash(b"abcd");
        assert_eq!(
            hash,
            [
                b'a', b'b', b'c', b'd', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_hash_bits() {
        let buffer = [0b00000001, 0b01000010, 0b00000011];
        let mut hash_bits = HashBits::new(&buffer);
        // read 9 bits would like [0b________, 0b_10000010, 0b00000011]
        // return 0b000000010 = 2
        assert_eq!(hash_bits.next(9), Some(2));
        // read 9 bits would like [0b________, 0b_________, 0b__000011]
        // return 0b100001000 = 264;
        assert_eq!(hash_bits.next(9), Some(264));
        // read 6 bits would like [0b________, 0b_________, 0b________]
        // return 0b000011 = 3;
        assert_eq!(hash_bits.next(6), Some(3));
        assert_eq!(hash_bits.next(1), None);
    }
}

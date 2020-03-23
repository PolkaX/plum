// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod bitset;
mod config;
mod decode;
mod encode;
mod error;
mod traits;

pub use self::decode::rle_decode;
pub use self::encode::rle_encode;
pub use self::error::RleDecodeError;

#[cfg(test)]
mod tests {
    use super::bitset::*;
    use super::*;

    fn assert_equal(expect_list: Vec<u8>) {
        let mut content = DynamicBitSet::new();
        let mut test_bit = vec![];
        for (index, exc) in expect_list.iter().enumerate() {
            for i in 0..8 {
                let d = ((*exc >> i) & 1) != 0;
                if d {
                    test_bit.push(index * BYTE_LEN as usize + i as usize);
                }
                content.push(d);
            }
        }

        for pos in test_bit.iter() {
            assert!(content.bit(*pos));
        }
        let mut r = content.find_next(0);
        for expect in test_bit.iter() {
            if *expect == 0 {
                continue;
            }
            let res = r.unwrap();
            assert_eq!(res, *expect);
            r = content.find_next(*expect);
        }
        assert_eq!(r, None);

        let v: Vec<u8> = content.into();
        assert_eq!(v, expect_list);
    }

    #[test]
    fn test_bit_content() {
        let except = vec![0b100_0010 as u8, 0b001_0100];
        assert_equal(except);

        let except = vec![0b0 as u8, 0b001_0100];
        assert_equal(except);

        let except = vec![0b1111_1111 as u8, 0b001_0100];
        assert_equal(except);

        let except = vec![0b0 as u8, 0b001_0100, 0b0101_1010];
        assert_equal(except);
    }

    #[test]
    fn test_generic() {
        use std::collections::BTreeSet;
        let _ = rle_encode(BTreeSet::<u64>::new().iter());
        let _ = rle_encode(BTreeSet::<u32>::new().iter());
        let _ = rle_encode(BTreeSet::<usize>::new().iter());
        let _ = rle_encode(BTreeSet::<u16>::new().iter());
    }

    macro_rules! set (
        { $($value:expr),+ } => {
            {
                let mut m = ::std::collections::BTreeSet::<u64>::new();
                $(
                    m.insert($value);
                )+
                m
            }
         };
    );

    #[test]
    fn test() {
        let test_case = vec![
            (set!(0, 100, 1000), vec![204_u8, 88, 6, 15, 2]),
            (::std::collections::BTreeSet::<u64>::new(), vec![0]),
            (set!(0), vec![12]),
            (set!(1), vec![24]),
            (
                set!(1000, 10_000_000_000),
                vec![0, 253, 32, 151, 192, 175, 160, 37, 1],
            ),
            (set!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10), vec![116, 1]),
            (set!(10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0), vec![116, 1]),
            (set!(10, 5, 0), vec![44, 149, 2]),
            (
                set!(100_000_000_000, 2_000_000, 3),
                vec![112, 194, 143, 168, 151, 127, 227, 112, 97, 122, 129],
            ),
        ];

        for (v, e) in test_case {
            test_roundtrip(v, e);
        }
    }

    fn test_roundtrip(set: std::collections::BTreeSet<u64>, expect: Vec<u8>) {
        let r = rle_encode(set.iter());
        assert_eq!(r, expect);
        let new: Vec<u64> = rle_decode::<u64>(r).unwrap();
        let s = new.into_iter().collect::<std::collections::BTreeSet<_>>();
        assert_eq!(set, s);
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub mod cbor;
pub mod json;

use num_bigint::{BigInt, Sign};
use num_traits::{Signed, ToPrimitive, Zero};

const SIZE_UNITS: [&str; 8] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];

/// Convert BigInt into size mod, like "0 B", "1.95 KiB" and "5 MiB", etc...
pub fn bigint_size_str(size: &BigInt) -> String {
    let (sign, mut size) = (size.sign(), size.abs());
    let mut i = 0;
    let mut decimal = BigInt::zero();
    let unit = BigInt::from(1024_u64);
    let mask = BigInt::from(1023_u64);
    while size >= unit && i + 1 < SIZE_UNITS.len() {
        decimal = size.clone() & mask.clone();
        size >>= 10;
        i += 1;
    }
    if decimal.is_zero() {
        if sign == Sign::Minus {
            format!("-{} {}", size, SIZE_UNITS[i])
        } else {
            format!("{} {}", size, SIZE_UNITS[i])
        }
    } else {
        let size = size.to_f64().unwrap();
        let part = decimal.to_f64().unwrap();
        let out = part / 1024_f64 + size;
        if sign == Sign::Minus {
            format!("-{:0.3} {}", out, SIZE_UNITS[i])
        } else {
            format!("{:0.3} {}", out, SIZE_UNITS[i])
        }
    }
}

#[test]
fn test_size_str() {
    let cases = vec![
        (0_u128, "0 B"),
        (1, "1 B"),
        (1024, "1 KiB"),
        (2000, "1.953 KiB"),
        (5 << 20, "5 MiB"),
        (11 << 60, "11 EiB"),
    ];
    for (num, expect) in cases {
        let big_int = BigInt::from(num);
        let size = bigint_size_str(&big_int);
        assert_eq!(size, expect);
    }

    let mut big_int = BigInt::from(50000_u64);
    big_int <<= 70;
    assert_eq!(bigint_size_str(&big_int), "50000 ZiB");

    let cases = vec![
        (0_i128, "0 B"),
        (-1, "-1 B"),
        (-1024, "-1 KiB"),
        (-2000, "-1.953 KiB"),
        (-5 << 20, "-5 MiB"),
        (-11 << 60, "-11 EiB"),
    ];
    for (num, expect) in cases {
        let big_int = BigInt::from(num);
        let size = bigint_size_str(&big_int);
        assert_eq!(size, expect);
    }

    let mut big_int = BigInt::from(-50000_i64);
    big_int <<= 70;
    assert_eq!(bigint_size_str(&big_int), "-50000 ZiB");
}

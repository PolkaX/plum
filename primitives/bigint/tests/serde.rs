// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_bigint::{BigInt, BigIntWrapper, BigUint, BigUintWrapper};

#[test]
fn big_int_cbor_serde() {
    let cases = vec![
        ("0", vec![64]),
        ("1", vec![66, 0, 1]),
        ("10", vec![66, 0, 10]),
        ("-10", vec![66, 1, 10]),
        ("9999", vec![67, 0, 39, 15]),
        (
            "12345678901234567891234567890123456789012345678901234567890",
            vec![
                88, 26, 0, 1, 247, 126, 230, 227, 172, 105, 112, 56, 202, 66, 148, 9, 33, 254, 186,
                53, 220, 190, 84, 150, 206, 63, 10, 210,
            ],
        ),
    ];
    for (s, expect) in cases {
        let int = s.parse::<BigInt>().unwrap();
        let cbor_wrapper = BigIntWrapper::from(int.clone());
        let ser = minicbor::to_vec(&cbor_wrapper).unwrap();
        assert_eq!(ser, expect);
        let de = minicbor::decode::<BigIntWrapper>(&ser).unwrap();
        assert_eq!(de.into_inner(), int);
    }
}

#[test]
fn big_uint_cbor_serde() {
    let cases = vec![
        ("0", vec![64]),
        ("1", vec![66, 0, 1]),
        ("10", vec![66, 0, 10]),
        ("9999", vec![67, 0, 39, 15]),
        (
            "12345678901234567891234567890123456789012345678901234567890",
            vec![
                88, 26, 0, 1, 247, 126, 230, 227, 172, 105, 112, 56, 202, 66, 148, 9, 33, 254, 186,
                53, 220, 190, 84, 150, 206, 63, 10, 210,
            ],
        ),
    ];
    for (s, expect) in cases {
        let uint = s.parse::<BigUint>().unwrap();
        let cbor_wrapper = BigUintWrapper::from(uint.clone());
        let ser = minicbor::to_vec(cbor_wrapper).unwrap();
        assert_eq!(ser, expect);
        let de = minicbor::decode::<BigUintWrapper>(&ser).unwrap();
        assert_eq!(de.into_inner(), uint);
    }
}

#[test]
fn big_int_json_serde() {
    let cases = vec![
        ("0", "\"0\""),
        ("1", "\"1\""),
        ("10", "\"10\""),
        ("-10", "\"-10\""),
        ("9999", "\"9999\""),
        (
            "12345678901234567891234567890123456789012345678901234567890",
            "\"12345678901234567891234567890123456789012345678901234567890\"",
        ),
    ];
    for (s, expect) in cases {
        let int = s.parse::<BigInt>().unwrap();
        let json_wrapper = BigIntWrapper::from(int.clone());
        let ser = serde_json::to_string(&json_wrapper).unwrap();
        assert_eq!(ser, expect);
        let de = serde_json::from_str::<BigIntWrapper>(&ser).unwrap();
        assert_eq!(de.into_inner(), int);
    }
}

#[test]
fn big_uint_json_serde() {
    let cases = vec![
        ("0", "\"0\""),
        ("1", "\"1\""),
        ("10", "\"10\""),
        ("9999", "\"9999\""),
        (
            "12345678901234567891234567890123456789012345678901234567890",
            "\"12345678901234567891234567890123456789012345678901234567890\"",
        ),
    ];
    for (s, expect) in cases {
        let uint = s.parse::<BigUint>().unwrap();
        let json_wrapper = BigUintWrapper::from(uint.clone());
        let ser = serde_json::to_string(&json_wrapper).unwrap();
        assert_eq!(ser, expect);
        let de = serde_json::from_str::<BigUintWrapper>(&ser).unwrap();
        assert_eq!(de.into_inner(), uint);
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde_derive::{Deserialize, Serialize};

use plum_bigint::{bigint_cbor, bigint_json, biguint_cbor, biguint_json, BigInt, BigUint};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct TestCborBigInt(#[serde(with = "bigint_cbor")] BigInt);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct TestCborBigUint(#[serde(with = "biguint_cbor")] BigUint);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct TestJsonBigInt(#[serde(with = "bigint_json")] BigInt);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct TestJsonBigUint(#[serde(with = "biguint_json")] BigUint);

#[test]
fn test_cbor_big_int() {
    use std::str::FromStr;
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
        let big_int = TestCborBigInt(BigInt::from_str(s).unwrap());
        let cbor = serde_cbor::to_vec(&big_int).unwrap();
        assert_eq!(cbor, expect);
        let out: TestCborBigInt = serde_cbor::from_slice(&cbor).unwrap();
        assert_eq!(big_int, out);
    }
}

#[test]
fn test_cbor_big_uint() {
    use std::str::FromStr;
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
        let big_uint = TestCborBigUint(BigUint::from_str(s).unwrap());
        let cbor = serde_cbor::to_vec(&big_uint).unwrap();
        assert_eq!(cbor, expect);
        let out: TestCborBigUint = serde_cbor::from_slice(&cbor).unwrap();
        assert_eq!(big_uint, out);
    }
}

#[test]
fn test_json_big_int() {
    use std::str::FromStr;
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
        let big_int = TestJsonBigInt(BigInt::from_str(s).unwrap());
        let json = serde_json::to_string(&big_int).unwrap();
        assert_eq!(json, expect);
        let out: TestJsonBigInt = serde_json::from_str(&json).unwrap();
        assert_eq!(big_int, out);
    }
}

#[test]
fn test_json_big_uint() {
    use std::str::FromStr;
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
        let big_uint = TestJsonBigUint(BigUint::from_str(s).unwrap());
        let json = serde_json::to_string(&big_uint).unwrap();
        assert_eq!(json, expect);
        let out: TestJsonBigUint = serde_json::from_str(&json).unwrap();
        assert_eq!(big_uint, out);
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! BigInt and BigUint with CBOR and JSON serialization/deserialization
//!
//! Notice, thought the `lotus` (the go version of filecoin) support `big.Int`
//! with cbor serialize/deserialize, in fact it only supports unsigned int.

#![deny(missing_docs)]

mod bigint;
mod biguint;

pub use num_bigint::{self, BigInt, BigUint, Sign};
pub use num_traits;

pub use self::bigint::bigint_size_str;
pub use self::bigint::BigIntWrapper;
// pub use self::bigint::cbor as bigint_cbor;
pub use self::bigint::json as bigint_json;

pub use self::biguint::biguint_size_str;
pub use self::biguint::BigUintWrapper;
// pub use self::biguint::cbor as biguint_cbor;
pub use self::biguint::json as biguint_json;

// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

const SIGNATUREMAXLENGTH: u32 = 200;

const KTSECP256K1: &str = "secp256k1";
const KTBLS: &str = "bls";

const IKTUNKNOWN: i32 = -1;
const IKTSECP256K1: i32 = 0;
const IKTBLS: i32 = 1;

#[derive(Default, PartialEq, Clone, Eq)]
pub struct Signature {
    ty: String,
    data: Vec<u8>,
}

// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::bigint::BigInt;

pub type FIL = BigInt;
pub type Bytes = [u8; 16];

pub trait Coin {
    fn bytes(&mut self) -> Bytes;
}

impl Coin for FIL {
    fn bytes(&mut self) -> Bytes {
        self.to_be_bytes()
    }
}

pub fn parse_fil(bytes: Bytes) -> FIL {
    u128::from_be_bytes(bytes)
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use bytes::Bytes;
use plum_bigint::BigUint;

pub type FIL = BigUint;

pub trait Coin {
    fn bytes(&mut self) -> Bytes;
}

impl Coin for FIL {
    fn bytes(&mut self) -> Bytes {
        self.to_bytes_be().into()
    }
}

pub fn parse_fil(bytes: &Bytes) -> FIL {
    FIL::from_bytes_be(bytes.as_ref())
}

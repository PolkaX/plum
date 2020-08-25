// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Error;

pub struct RlePlusDecoder<'a> {
    bytes: &'a [u8],
    next_byte: u8,
    bits: u16,
    num_bits: u32,
}

impl<'a> RlePlusDecoder<'a> {
    ///
    pub fn new(bytes: &'a [u8]) -> Self {
        todo!()
    }

    ///
    pub fn decode(&mut self, num_bits: u32) -> u8 {
        todo!()
    }

    ///
    pub fn decode_varint(&mut self) -> Result<usize, Error> {
        todo!()
    }

    ///
    pub fn decode_len(&mut self) -> Result<Option<usize>, Error> {
        todo!()
    }
}

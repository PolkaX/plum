// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

const SMALL_BLOCK_LENGTH: usize = 0x04;

///
#[derive(Default)]
pub struct BitStreamEncoder {
    /// The buffer that is written to.
    bytes: Vec<u8>,
    /// The most recently written bits. Whenever this exceeds 8 bits, one byte is written to `bytes`.
    bits: u16,
    /// The number of bits currently stored in `bits`.
    num_bits: u32,
}

impl BitStreamEncoder {
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Write a given number of bits from `byte` to the buffer.
    pub fn encode(&mut self, byte: u8, num_bits: u32) {
        todo!()
    }

    ///
    pub fn encode_len(&mut self, len: usize) {
        todo!()
    }
}

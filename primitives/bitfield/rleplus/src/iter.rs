// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::error::Error;
use crate::decoder::BitStreamDecoder;

const RLE_PLUS_VERSION: u8 = 0;

pub struct Run<'a> {
    reader: BitStreamDecoder<'a>,
    next_value: bool,
}

impl<'a> Run<'a> {
    ///
    pub fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        let mut decoder = BitStreamDecoder::new(bytes);

        let version = decoder.decode(2);
        if version != RLE_PLUS_VERSION {
            return Err(Error::WrongVersion);
        }

        let next_value = decoder.decode(1) == 1;

        Ok(Self { reader, next_value })
    }
}

impl<'a> Iterator for Run<'a> {
    type Item = Result<(bool, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = match self.reader.decode_len() {
            Ok(len) => len.unwrap(),
            Err(err) => return Some(Err(err)),
        };

        let run = (self.next_value, len);
        self.next_value = !self.next_value;
        Some(Ok(run))
    }
}

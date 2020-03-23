// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::ops::{Deref, DerefMut};

use crate::bitset::DynamicBitSet;
use crate::config;
use crate::error::*;
use crate::traits::{Cast, Number};

struct BitSetHelper {
    bitset: DynamicBitSet,
    index: usize,
    magnitude: bool,
}

impl BitSetHelper {
    fn new(bitset: DynamicBitSet) -> Self {
        BitSetHelper {
            bitset,
            index: 0,
            magnitude: false,
        }
    }
}

impl Deref for BitSetHelper {
    type Target = DynamicBitSet;

    fn deref(&self) -> &Self::Target {
        &self.bitset
    }
}

impl DerefMut for BitSetHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bitset
    }
}

///
pub fn rle_decode<Item: Number>(data: Vec<u8>) -> Result<Vec<Item>> {
    let content: DynamicBitSet = data.into();
    let helper = &mut BitSetHelper::new(content);

    let two: Item = get_span(helper, 2)?;
    if helper.size() < config::SMALL_BLOCK_LENGTH || (two == Item::one()) {
        return Err(RleDecodeError::VersionMismatch);
    }
    let one: Item = get_span(helper, 1)?;
    helper.magnitude = one == Item::one();

    let mut value = Item::zero();
    let mut output = vec![];
    while helper.find_next(helper.index - 1).is_some() {
        let header: Item = get_span(helper, 1)?;
        if header == Item::one() {
            decode_single_block(helper, &mut value, &mut output);
        } else if header == Item::zero() {
            let block_header: Item = get_span(helper, 1)?;
            if block_header == Item::zero() {
                decode_long_block(helper, &mut value, &mut output)?;
            } else {
                decode_small_block(helper, &mut value, &mut output)?;
            }
        }
    }
    let max_size = config::OBJECT_MAX_SIZE / std::mem::size_of::<Item>();
    if output.len() > max_size {
        return Err(RleDecodeError::MaxSizeExceed);
    }

    Ok(output)
}

fn get_span<Item: Number>(helper: &mut BitSetHelper, count: usize) -> Result<Item> {
    let end = helper.index + count;
    if helper.size() < end {
        return Err(RleDecodeError::DataIndexFailure);
    }
    let mut value = Item::zero();
    let mut shift = Item::zero();
    for i in helper.index..end {
        let slice = if helper.bit(i) {
            Item::one()
        } else {
            Item::zero()
        };
        value |= slice << shift;
        shift += Item::one();
    }
    helper.index += count;
    Ok(value)
}

fn decode_single_block<Item: Number>(
    helper: &mut BitSetHelper,
    current_value: &mut Item,
    output: &mut Vec<Item>,
) {
    if helper.magnitude {
        output.push(*current_value);
    }
    helper.magnitude = !helper.magnitude;
    *current_value += Item::one();
}

fn decode_small_block<Item: Number>(
    helper: &mut BitSetHelper,
    current_value: &mut Item,
    output: &mut Vec<Item>,
) -> Result<()> {
    let length: Item = get_span(helper, config::SMALL_BLOCK_LENGTH)?;
    if helper.magnitude {
        for _ in 0_usize..length.into() {
            output.push(*current_value);
            *current_value += Item::one();
        }
    } else {
        *current_value += length;
    }
    helper.magnitude = !helper.magnitude;
    Ok(())
}

fn decode_long_block<Item: Number>(
    helper: &mut BitSetHelper,
    current_value: &mut Item,
    output: &mut Vec<Item>,
) -> Result<()> {
    // let mut slice: u8 = 0;
    let mut bytes: Vec<u8> = vec![];
    loop {
        let slice: u8 = get_span::<Item>(helper, config::BYTE_BITS_COUNT)?.into();
        bytes.push(slice);
        if (slice & config::BYTE_SLICE_VALUE as u8) == 0 {
            break;
        }
    }
    let length: Item = unpack(bytes)?;
    if helper.magnitude {
        for _ in 0_usize..length.into() {
            output.push(*current_value);
            *current_value += Item::one();
        }
    } else {
        *current_value += length;
    }
    helper.magnitude = !helper.magnitude;
    Ok(())
}

fn unpack<Item: Number>(data: Vec<u8>) -> Result<Item> {
    let max_shift = std::mem::size_of::<Item>() * config::BYTE_BITS_COUNT;
    let mut shift: usize = 0;
    let mut value = Item::zero();
    for byte in data {
        if shift > max_shift {
            return Err(RleDecodeError::UnpackOverflow);
        }
        let byte = byte as usize;
        if byte < config::BYTE_SLICE_VALUE {
            value |= Cast::from(byte << shift);
            break;
        }

        value |= Cast::from((byte & config::UNPACK_BYTE_MASK) << shift);
        shift += config::PACK_BYTE_SHIFT;
    }
    Ok(value)
}

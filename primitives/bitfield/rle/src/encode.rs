// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::iter::Iterator;
use std::ops::Deref;

use crate::bitset::{DynamicBitSet, BYTE_LEN};
use crate::config;
use crate::traits::{Cast, Number};

///
pub fn encode<Item, T, I>(input: I) -> Vec<u8>
where
    T: Deref<Target = Item>,
    Item: Number,
    I: Iterator<Item = T>,
{
    let mut content = DynamicBitSet::new();
    let (first, periods) = get_periods(input);
    init_content(&mut content);

    let flag = first.map(|i| i.is_zero()).unwrap_or(false);
    content.push(flag);
    for value in periods {
        if value == Item::one() {
            content.push(true);
        } else if value < Cast::from(config::LONG_BLOCK_VALUE) {
            push_small_block(&mut content, value);
        } else if value >= Cast::from(config::LONG_BLOCK_VALUE) {
            push_long_block(&mut content, value);
        }
    }

    content.into()
}

fn get_periods<Item, T, I>(mut input: I) -> (Option<Item>, Vec<Item>)
where
    T: Deref<Target = Item>,
    Item: Number,
    I: Iterator<Item = T>,
{
    let (first, mut prev) = input
        .next()
        .map(|i| (Some(*i), *i))
        .unwrap_or((None, Item::zero()));

    let mut periods = vec![];
    if first.is_none() {
        return (first, periods);
    }

    if !prev.is_zero() {
        periods.push(prev);
    }

    periods.push(Item::one());
    for item in input {
        let diff: Item = *item - prev;
        if diff.is_one() {
            if let Some(last) = periods.last_mut() {
                *last += Item::one();
            }
        } else if diff > Item::one() {
            periods.push(diff - Item::one());
            periods.push(Item::one());
        }
        prev = *item;
    }
    (first, periods)
}

fn init_content(content: &mut DynamicBitSet) {
    content.clear();
    content.push(false);
    content.push(false);
}

fn push_small_block<Item: Number>(content: &mut DynamicBitSet, block: Item) {
    content.push(false);
    content.push(true);
    // let i: Item = 1_usize.into();
    for i in 0..config::SMALL_BLOCK_LENGTH {
        let i = Cast::from(i);
        let c = (block & (Item::one() << i)) >> i == Item::one();
        content.push(c);
    }
}

fn push_long_block<Item: Number>(content: &mut DynamicBitSet, block: Item) {
    content.push(false);
    content.push(false);
    let mut slice = block;
    while slice >= Cast::from(config::BYTE_SLICE_VALUE) {
        let byte: u8 = (slice | Cast::from(config::BYTE_SLICE_VALUE)).into();
        push_byte(content, byte);
        slice >>= Cast::from(config::PACK_BYTE_SHIFT);
    }
    push_byte(content, slice.into())
}

fn push_byte(content: &mut DynamicBitSet, byte: u8) {
    for i in 0_u8..BYTE_LEN {
        let bit = ((byte >> i) & 1) != 0;
        content.push(bit);
    }
}

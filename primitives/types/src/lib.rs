// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The Common types and utils of primitives.

#![deny(missing_docs)]

mod actor;
/// JSON serialization/deserialization of Vec<u8> using base64,
/// in order to be compatible with golang standard library.
pub mod base64;
mod chain_epoch;

pub use self::actor::{json as actor_json, Actor};
pub use self::chain_epoch::{ChainEpoch, EpochDuration, CHAIN_EPOCH_UNDEFINED};

use plum_bigint::BigInt;
use plum_hash::H256;

///
pub type DealId = u64;
///
pub type DealWeight = BigInt;
///
pub type TokenAmount = BigInt;
///
pub type PeerId = String;
///
pub type ActorId = u64;
///
pub type SectorNumber = u64;
///
pub type SectorSize = u64;
///
pub fn readable_sector_size(s: SectorSize) -> String {
    const LIST: [&'static str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut unit = 0;
    let mut s = s;
    while s >= 1024 && unit < LIST.len() - 1 {
        s /= 1024;
        unit += 1;
    }
    format!("{}{}", s, LIST[unit])
}

///
pub type Randomness = H256;

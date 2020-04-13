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

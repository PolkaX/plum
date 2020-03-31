// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

/// Actor with CBOR and JSON serialization/deserialization.
pub mod actor;
///
pub mod chain_epoch;

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
pub type ActorID = u64;
///
pub type SectorNumber = u64;
///
pub type SectorSize = u64;

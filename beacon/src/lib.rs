// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod beacon;
#[path = "../proto/mod.rs"]
mod proto;

pub use self::beacon::RandomBeacon;

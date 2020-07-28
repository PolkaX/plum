// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of the Drand Beacon client for providing randomness.

#![deny(missing_docs)]

extern crate bls_signatures as bls;

mod beacon;
mod mock;
mod proto;

pub use self::beacon::{DrandBeacon, RandomBeacon};
pub use self::mock::MockBeacon;

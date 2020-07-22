// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// The Drand gRPC interface.
#[allow(dead_code)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

pub use self::protos::*;

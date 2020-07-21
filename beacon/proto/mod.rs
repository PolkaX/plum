// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// The Drand gRPC interface.
pub mod drand {
    include!(concat!(env!("OUT_DIR"), "/drand.rs"));
}

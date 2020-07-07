// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The p2p implementation about the fil.

#![deny(missing_docs)]

#[macro_use]
extern crate log;

mod behaviour;
mod config;
mod protocol;
mod service;

pub use self::behaviour::{Behaviour, BehaviourEvent};
pub use self::config::Libp2pConfig;
pub use self::protocol::{
    BlockSyncCodec, BlockSyncProtocolName, BlockSyncRequest, BlockSyncResponse, BlockSyncTipset,
    BLOCKSYNC_PROTOCOL_ID,
};
pub use self::protocol::{
    HelloCodec, HelloProtocolName, HelloRequest, HelloResponse, HELLO_PROTOCOL_ID,
};
pub use self::service::{build_transport, generate_new_keypair, Libp2pEvent, Libp2pService};

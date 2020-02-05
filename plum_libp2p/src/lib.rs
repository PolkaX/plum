// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub mod behaviour;
pub mod config;
pub mod rpc;
pub mod service;

pub use config::Libp2pConfig;
// Reexport for avoiding the multiple version issues.
pub use libp2p::gossipsub::{MessageId, TopicHash};
pub use libp2p::{Multiaddr, PeerId};

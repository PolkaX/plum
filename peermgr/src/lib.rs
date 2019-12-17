// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use std::{collections::HashMap, time::Instant};

use libp2p::{Multiaddr, PeerId};

const MAX_FIL_PEERS: u32 = 32;
const MIN_FIL_PEERS: u32 = 8;

#[derive(Debug, Clone)]
pub struct Node {}

#[derive(Debug, Clone)]
pub struct PeerMgr {
    bootstrappers: Vec<Multiaddr>,
    peers: HashMap<PeerId, Node>,
    max_fil_peers: u32,
    min_fil_peers: u32,
    expanding: bool,
    //   swarm: Swarm,
}

/*
type Swarm =
libp2p::swarm::ExpandedSwarm<impl std::clone::Clone+libp2p::core::transport::Transport, Behaviour<libp2p::core::muxing::SubstreamRef<std::sync::Arc<impl std::marker::Send+std::marker::Sync+libp2p::core::muxing::StreamMuxer>>>, libp2p::core::either::EitherOutput<libp2p::floodsub::protocol::FloodsubRpc, libp2p::kad::handler::KademliaHandlerIn<libp2p::kad::query::QueryId>>, libp2p::core::either::EitherOutput<libp2p::floodsub::layer::InnerMessage, libp2p::kad::handler::KademliaHandlerEvent<libp2p::kad::query::QueryId>>, libp2p::swarm::protocols_handler::select::IntoProtocolsHandlerSelect<libp2p::swarm::protocols_handler::one_shot::OneShotHandler<libp2p::core::muxing::SubstreamRef<std::sync::Arc<impl std::marker::Send+std::marker::Sync+libp2p::core::muxing::StreamMuxer>>, libp2p::floodsub::protocol::FloodsubConfig, libp2p::floodsub::protocol::FloodsubRpc, libp2p::floodsub::layer::InnerMessage>, libp2p::kad::handler::KademliaHandler<libp2p::core::muxing::SubstreamRef<std::sync::Arc<impl std::marker::Send+std::marker::Sync+libp2p::core::muxing::StreamMuxer>>, libp2p::kad::query::QueryId>>, libp2p::core::either::EitherError<libp2p::swarm::protocols_handler::ProtocolsHandlerUpgrErr<std::io::Error>, std::io::Error>>
;*/

#[cfg(test)]
mod tests {}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::io;
use std::time::Duration;

use libp2p::{
    core::{
        identity::Keypair,
        multiaddr::Multiaddr,
        muxing::StreamMuxerBox,
        transport::{boxed::Boxed, Transport},
        upgrade, PeerId,
    },
    dns, mplex,
    request_response::{RequestId, ResponseChannel},
    secio,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux,
};

use crate::behaviour::{Behaviour, BehaviourEvent};
use crate::config::Libp2pConfig;
use crate::protocol::{BlockSyncRequest, BlockSyncResponse};
use crate::protocol::{HelloRequest, HelloResponse};

/// The types of events than can be obtained from polling the libp2p service.
///
/// This is a subset of the events that a libp2p swarm emits.
#[derive(Debug)]
pub enum Libp2pEvent {
    /// A behaviour event
    Behaviour(BehaviourEvent),
    /// A new listening address has been established.
    NewListenAddr(Multiaddr),
}

/// The configuration and state of the libp2p components.
pub struct Libp2pService {
    /// The libp2p Swarm handler.
    pub swarm: Swarm<Behaviour>,
}

impl Libp2pService {
    /// Build libp2p service given the libp2p config.
    pub fn new(local_key_pair: Keypair, config: Libp2pConfig) -> Self {
        let local_peer_id = local_key_pair.public().into_peer_id();
        info!("Local peer id: {}", local_peer_id);

        let mut swarm = {
            let transport = build_transport(local_key_pair.clone());
            let behaviour = Behaviour::new(local_key_pair, &config);
            Swarm::new(transport, behaviour, local_peer_id)
        };

        Swarm::listen_on(&mut swarm, config.listen_address).unwrap();

        // Subscribe to gossipsub topics.
        for topic in config.pubsub_topics {
            if swarm.subscribe(topic.clone()) {
                info!("Subscribe to topic: {}", topic);
            } else {
                warn!("Couldn't subscribe to topic: {}", topic);
            }
        }

        for node in config.boot_nodes {
            match Swarm::dial_addr(&mut swarm, node.clone()) {
                Ok(_) => info!("Dialed libp2p peer address: {}", node),
                Err(err) => warn!("Dial address {} failed: {}", node, err),
            }
        }

        Self { swarm }
    }

    /// Sends a hello request to a peer, return a request Id.
    pub fn send_hello_request(&mut self, peer: &PeerId, request: HelloRequest) -> RequestId {
        self.swarm.send_hello_request(peer, request)
    }

    /// Sends a hello response to a peer over the channel.
    pub fn send_hello_response(
        &mut self,
        channel: ResponseChannel<HelloResponse>,
        response: HelloResponse,
    ) {
        self.swarm.send_hello_response(channel, response)
    }

    /// Sends a blocksync request to a peer, return a request Id.
    pub fn send_blocksync_request(
        &mut self,
        peer: &PeerId,
        request: BlockSyncRequest,
    ) -> RequestId {
        self.swarm.send_blocksync_request(peer, request)
    }

    /// Sends a blocksync response to a peer over the channel.
    pub fn send_blocksync_response(
        &mut self,
        channel: ResponseChannel<BlockSyncResponse>,
        response: BlockSyncResponse,
    ) {
        self.swarm.send_blocksync_response(channel, response)
    }

    /// Returns the next event that happens in the `Swarm`.
    pub async fn next_event(&mut self) -> Libp2pEvent {
        loop {
            match self.swarm.next_event().await {
                SwarmEvent::Behaviour(behaviour) => return Libp2pEvent::Behaviour(behaviour),
                // A connection could be established with a banned peer.
                // This is handled inside the behaviour.
                SwarmEvent::ConnectionEstablished { .. } => {}
                SwarmEvent::ConnectionClosed {
                    peer_id,
                    endpoint,
                    num_established,
                    cause,
                } => {
                    debug!(
                        "Connection closed (peer: {}, endpoint: {:?}, num_established: {}): {:?}",
                        peer_id, endpoint, num_established, cause
                    );
                }
                SwarmEvent::IncomingConnection {
                    local_addr,
                    send_back_addr,
                } => {
                    debug!(
                        "Incoming connection (local_addr: {}, send_back_addr: {})",
                        local_addr, send_back_addr
                    );
                }
                SwarmEvent::IncomingConnectionError {
                    local_addr,
                    send_back_addr,
                    error,
                } => {
                    debug!(
                        "Incoming connection (local_addr: {}, send_back_addr: {}) error: {}",
                        local_addr, send_back_addr, error
                    );
                }
                // We do not ban peers at the swarm layer, so this should never occur.
                SwarmEvent::BannedPeer { .. } => {}
                SwarmEvent::UnreachableAddr {
                    peer_id,
                    address,
                    error,
                    attempts_remaining,
                } => {
                    debug!(
                        "Dial an address (peer_id: {}, address: {}, attempts_remaining: {}) error: {}",
                        peer_id, address, attempts_remaining, error
                    );
                }
                SwarmEvent::UnknownPeerUnreachableAddr { address, error } => {
                    debug!("Peer not known at dialed address {} : {}", address, error);
                }
                SwarmEvent::NewListenAddr(multiaddr) => {
                    return Libp2pEvent::NewListenAddr(multiaddr)
                }
                SwarmEvent::ExpiredListenAddr(multiaddr) => {
                    debug!("Listen address {} expired", multiaddr);
                }
                SwarmEvent::ListenerClosed { addresses, reason } => {
                    debug!("Listener close (addresses: {:?}): {:?}", addresses, reason);
                }
                SwarmEvent::ListenerError { error } => {
                    debug!("Listener error: {}", error);
                }
                SwarmEvent::Dialing(peer_id) => {
                    debug!("Dialing peer {}", peer_id);
                }
            }
        }
    }
}

/// Builds the transport that serves as a common ground for all connections.
pub fn build_transport(local_key_pair: Keypair) -> Boxed<(PeerId, StreamMuxerBox), io::Error> {
    let transport = tcp::TcpConfig::new().nodelay(true);
    let transport = dns::DnsConfig::new(transport).unwrap();

    transport
        .upgrade(upgrade::Version::V1)
        .authenticate(secio::SecioConfig::new(local_key_pair))
        .multiplex(upgrade::SelectUpgrade::new(
            yamux::Config::default(),
            mplex::MplexConfig::new(),
        ))
        .map(|(peer, muxer), _endpoint| (peer, StreamMuxerBox::new(muxer)))
        .timeout(Duration::from_secs(20))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        .boxed()
}

/// Generate a new libp2p ed25519 keypair.
pub fn generate_new_keypair() -> Keypair {
    info!("Generated new keypair!");
    Keypair::generate_ed25519()
}

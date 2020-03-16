// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use futures::{Async, Stream};
use libp2p::{
    core,
    core::muxing::StreamMuxerBox,
    core::nodes::Substream,
    core::transport::boxed::Boxed,
    gossipsub::{MessageId, TopicHash},
    identity::Keypair,
    mplex, secio, yamux, Multiaddr, PeerId, Swarm, Transport,
};
use log::{error, info};
use std::io::{Error, ErrorKind};
use std::time::Duration;

use crate::behaviour::{Behaviour, BehaviourEvent};
use crate::config::Libp2pConfig;
use crate::rpc::RPCEvent;

type Libp2pStream = Boxed<(PeerId, StreamMuxerBox), Error>;
type Libp2pBehaviour = Behaviour<Substream<StreamMuxerBox>>;

/// The Libp2pService listens to events from the LIBP2P swarm.
pub struct Libp2pService {
    pub swarm: Swarm<Libp2pStream, Libp2pBehaviour>,
}

impl Libp2pService {
    /// Build libp2p service given the libp2p config.
    pub fn new(config: &Libp2pConfig) -> Self {
        let net_keypair = generate_new_keypair();
        let peer_id = PeerId::from(net_keypair.public());

        info!("Local peer id: {:?}", peer_id);

        let transport = build_transport(net_keypair.clone());

        let mut swarm = {
            let behaviour = Behaviour::new(&net_keypair);
            Swarm::new(transport, behaviour, peer_id.clone())
        };

        // helper closure for dialing peers
        let mut dial_addr = |multiaddr: Multiaddr| {
            match Swarm::dial_addr(&mut swarm, multiaddr.clone()) {
                Ok(()) => {
                    info!("Dialing libp2p peer address: {}", multiaddr);
                }
                Err(err) => error!(
                    "Could not connect to peer, address {}, error: {:?}",
                    multiaddr, err
                ),
            };
        };

        for node in config.bootnodes.clone() {
            dial_addr(node);
        }

        Swarm::listen_on(&mut swarm, config.listen_address.clone())
            .expect(&format!("Failed to listen on {}", config.listen_address));

        swarm
            .kad
            .add_address(&peer_id, config.listen_address.clone());

        for topic in config.pubsub_topics.clone() {
            swarm.subscribe(topic);
        }

        Self { swarm }
    }
}

impl Stream for Libp2pService {
    type Item = Libp2pEvent;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        loop {
            match self.swarm.poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    BehaviourEvent::DiscoveredPeer(peer) => {
                        libp2p::Swarm::dial(&mut self.swarm, peer);
                    }
                    BehaviourEvent::HelloSubscribed(peer) => {
                        return Ok(Async::Ready(Some(Libp2pEvent::HelloSubscribed(peer))));
                    }
                    BehaviourEvent::RPC(peer, rpc_event) => {
                        return Ok(Async::Ready(Some(Libp2pEvent::RPC(peer, rpc_event))));
                    }
                    BehaviourEvent::ExpiredPeer(_) => {}
                    BehaviourEvent::GossipMessage {
                        id,
                        source,
                        topics,
                        data,
                    } => {
                        return Ok(Async::Ready(Some(Libp2pEvent::PubsubMessage {
                            id,
                            source,
                            topics,
                            data,
                        })));
                    }
                },
                Ok(Async::Ready(None)) => break,
                Ok(Async::NotReady) => break,
                _ => break,
            }
        }
        Ok(Async::NotReady)
    }
}

/// LIBP2P event that will be delivered to the NetworkService.
pub enum Libp2pEvent {
    PubsubMessage {
        id: MessageId,
        source: PeerId,
        topics: Vec<TopicHash>,
        data: Vec<u8>,
    },
    HelloSubscribed(PeerId),
    RPC(PeerId, RPCEvent),
}

pub fn build_transport(local_key: Keypair) -> Boxed<(PeerId, StreamMuxerBox), Error> {
    let transport = libp2p::tcp::TcpConfig::new().nodelay(true);
    let transport = libp2p::dns::DnsConfig::new(transport);

    transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(secio::SecioConfig::new(local_key))
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux::Config::default(),
            mplex::MplexConfig::new(),
        ))
        .map(|(peer, muxer), _| (peer, core::muxing::StreamMuxerBox::new(muxer)))
        .timeout(Duration::from_secs(20))
        .map_err(|err| Error::new(ErrorKind::Other, err))
        .boxed()
}

/// Generate a new libp2p keypair
fn generate_new_keypair() -> Keypair {
    let generated_keypair = Keypair::generate_ed25519();
    info!("Generated new keypair!");

    // TODO: save to the disk

    generated_keypair
}

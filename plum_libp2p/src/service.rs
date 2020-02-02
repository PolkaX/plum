use super::behaviour::{Behaviour, BehaviourEvent};
use super::config::Libp2pConfig;
use futures::{Async, Stream};
use libp2p::{
    core,
    core::muxing::StreamMuxerBox,
    core::nodes::Substream,
    core::transport::boxed::Boxed,
    gossipsub::TopicHash,
    identity::{ed25519, Keypair},
    mplex, secio, yamux, Multiaddr, PeerId, Swarm, Transport,
};
use log::{debug, error, info, trace};
use std::io::{Error, ErrorKind};
use std::time::Duration;

type Libp2pStream = Boxed<(PeerId, StreamMuxerBox), Error>;
type Libp2pBehaviour = Behaviour<Substream<StreamMuxerBox>>;

/// The Libp2pService listens to events from the Libp2p swarm.
pub struct Libp2pService {
    pub swarm: Swarm<Libp2pStream, Libp2pBehaviour>,
}

impl Libp2pService {
    /// Constructs a Libp2pService
    pub fn new(config: &Libp2pConfig) -> Self {
        let net_keypair = get_keypair();
        let peer_id = PeerId::from(net_keypair.public());

        info!("Local peer id: {:?}", peer_id);

        let transport = build_transport(net_keypair.clone());

        let mut swarm = {
            let behaviour = Behaviour::new(&net_keypair);
            Swarm::new(transport, behaviour, peer_id.clone())
        };

        swarm
            .kad
            .add_address(&peer_id, config.listen_address.clone());

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

        Swarm::listen_on(&mut swarm, config.listen_address.clone()).unwrap();

        for topic in config.pubsub_topics.clone() {
            swarm.subscribe(topic);
        }

        Libp2pService { swarm }
    }
}

impl Stream for Libp2pService {
    type Item = Libp2pEvent;
    type Error = ();

    /// Continuously polls the Libp2p swarm to get events
    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        loop {
            match self.swarm.poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    BehaviourEvent::DiscoveredPeer(peer) => {
                        libp2p::Swarm::dial(&mut self.swarm, peer);
                    }
                    BehaviourEvent::HelloSubscribed(peer) => {
                        // TODO: say hello
                        println!("----- hello message from network: {:?}", peer);
                        return Ok(Async::Ready(Option::from(Libp2pEvent::Hello(peer))));
                    }
                    BehaviourEvent::ExpiredPeer(_) => {}
                    BehaviourEvent::GossipMessage {
                        source,
                        topics,
                        message,
                    } => {
                        println!(
                            "----- received gossipsub source:{:?}, topics:{:?}, message: {:?}",
                            source, topics, message
                        );
                        return Ok(Async::Ready(Option::from(Libp2pEvent::PubsubMessage {
                            source,
                            topics,
                            message,
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

/// Events emitted by this Service to be listened by the NetworkService.
#[derive(Clone)]
pub enum Libp2pEvent {
    PubsubMessage {
        source: PeerId,
        topics: Vec<TopicHash>,
        message: Vec<u8>,
    },
    Hello(PeerId),
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

/// Fetch keypair from disk, or generate a new one if its not available
fn get_keypair() -> Keypair {
    let path_to_keystore = "/Users/xlc/.forest/libp2p/keypair";
    return generate_new_peer_id();
    // let local_keypair = match read_file_to_vec(&path_to_keystore) {
    // Err(e) => {
    // info!(log, "Networking keystore not found!");
    // trace!(log, "Error {:?}", e);
    // return generate_new_peer_id(log);
    // }
    // Ok(mut vec) => {
    // // If decoding fails, generate new peer id
    // // TODO rename old file to keypair.old(?)
    // match ed25519::Keypair::decode(&mut vec) {
    // Ok(kp) => {
    // info!(log, "Recovered keystore from {:?}", &path_to_keystore);
    // kp
    // }
    // Err(e) => {
    // info!(log, "Could not decode networking keystore!");
    // trace!(log, "Error {:?}", e);
    // return generate_new_peer_id(log);
    // }
    // }
    // }
    // };

    // Keypair::Ed25519(local_keypair)
}

/// Generates a new libp2p keypair and saves to disk
fn generate_new_peer_id() -> Keypair {
    let path_to_keystore = "/Users/xlc/.forest/libp2p/";
    let generated_keypair = Keypair::generate_ed25519();
    log::info!("Generated new keystore!");

    if let Keypair::Ed25519(key) = generated_keypair.clone() {
        // if let Err(e) = write_to_file(&key.encode(), &path_to_keystore, "keypair") {
        // info!(log, "Could not write keystore to disk!");
        // trace!(log, "Error {:?}", e);
        // };
    }

    generated_keypair
}

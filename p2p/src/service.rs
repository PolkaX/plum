// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use libp2p::{core::PeerId, identity::Keypair, swarm::Swarm};

use crate::behaviour::Behaviour;
use crate::config::Libp2pConfig;
use crate::transport::build_transport;

/// The Libp2pService listens to events from the Libp2p swarm.
pub struct Libp2pService {
    pub swarm: Swarm<Behaviour>,
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
            Swarm::new(transport, behaviour, peer_id)
        };

        for node in &config.bootnodes {
            match Swarm::dial_addr(&mut swarm, node.clone()) {
                Ok(_) => info!("Dialed libp2p peer address: {}", node),
                Err(err) => warn!("Dial address {} failed: {}", node, err),
            }
        }

        Swarm::listen_on(&mut swarm, config.listen_address.clone())
            .expect(&format!("Failed to listen on {}", config.listen_address));

        for topic in config.pubsub_topics.clone() {
            swarm.subscribe(topic);
        }

        Self { swarm }
    }
}

/*
impl Stream for Libp2pService {
    type Item = Libp2pEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.swarm.poll_next(cx) {
                Poll::Ready(Some(event)) => match event {
                    BehaviourEvent::MdnsDiscoveredPeer(peer) => {
                        Swarm::dial(&mut self.swarm, &peer);
                    }
                    BehaviourEvent::MdnsExpiredPeer(_) => {}
                    BehaviourEvent::GossipsubMessage {
                        peer_id,
                        message_id,
                        data,
                        topics,
                    } => {
                        return Poll::Ready(Some(Libp2pEvent::GossipsubMessage {
                            peer_id,
                            message_id,
                            data,
                            topics,
                        }));
                    }
                    BehaviourEvent::GossipsubSubscribed { peer_id, topic } => {
                        if topic.as_str() == HELLO_TOPIC {
                            return Poll::Ready(Some(Libp2pEvent::GossipsubSubscribedHello(
                                peer_id,
                            )));
                        }
                    }
                },
                Poll::Ready(None) => break,
                Poll::Pending => break,
            }
        }
        Poll::Pending
    }
}

/// Libp2p event that will be delivered to the NetworkService.
pub enum Libp2pEvent {
    GossipsubMessage {
        peer_id: PeerId,
        message_id: MessageId,
        data: Vec<u8>,
        topics: Vec<TopicHash>,
    },
    GossipsubSubscribedHello(PeerId),
}
*/

// TODO: save to the disk
/// Generate a new libp2p keypair
fn generate_new_keypair() -> Keypair {
    info!("Generated new keypair!");
    Keypair::generate_ed25519()
}

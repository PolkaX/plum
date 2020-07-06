// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::task::{Context, Poll};

use libp2p::{
    core::{identity, PeerId},
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageId, Topic, TopicHash},
    identify::{Identify, IdentifyEvent},
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingEvent, PingFailure, PingSuccess},
    swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess, PollParameters},
    NetworkBehaviour,
};

// use crate::new_rpc::{Rpc, RpcEvent, RpcMessage};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BehaviourEvent", poll_method = "poll")]
pub struct Behaviour {
    pub ping: Ping,
    pub identify: Identify,
    pub mdns: Mdns,
    pub gossipsub: Gossipsub,
    // pub rpc: Rpc,
    #[behaviour(ignore)]
    events: Vec<BehaviourEvent>,
}

pub enum BehaviourEvent {
    MdnsDiscoveredPeer(PeerId),
    MdnsExpiredPeer(PeerId),
    GossipsubMessage {
        /// Id of the peer that published this message.
        peer_id: PeerId,
        /// Id of gossipsub message.
        message_id: MessageId,
        /// Content of the message. Its meaning is out of scope of this library.
        data: Vec<u8>,
        /// List of topics this message belongs to.
        topics: Vec<TopicHash>,
    },
    GossipsubSubscribed {
        /// Remote that has subscribed.
        peer_id: PeerId,
        /// The topic it has subscribed to.
        topic: TopicHash,
    },
    GossipsubUnsubscribed {
        /// Remote that has subscribed.
        peer_id: PeerId,
        /// The topic it has subscribed to.
        topic: TopicHash,
    },
    // RpcPeerDialed(PeerId),
    // RpcPeerDisconnected(PeerId),
    // RpcMessage(PeerId, RpcMessage),
}

impl NetworkBehaviourEventProcess<PingEvent> for Behaviour {
    fn inject_event(&mut self, event: PingEvent) {
        match event.result {
            Ok(PingSuccess::Ping { rtt }) => {
                debug!(
                    "PingSuccess::Ping rtt to {} is {} ms",
                    event.peer.to_base58(),
                    rtt.as_millis()
                );
            }
            Ok(PingSuccess::Pong) => {
                debug!("PingSuccess::Pong from {}", event.peer.to_base58());
            }
            Err(PingFailure::Timeout) => {
                debug!("PingFailure::Timeout {}", event.peer.to_base58());
            }
            Err(PingFailure::Other { error }) => {
                debug!("PingFailure::Other {}: {}", event.peer.to_base58(), error);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<IdentifyEvent> for Behaviour {
    fn inject_event(&mut self, event: IdentifyEvent) {
        match event {
            IdentifyEvent::Received {
                peer_id,
                info,
                observed_addr,
            } => {
                debug!("Identified Peer {:?}", peer_id);
                debug!("protocol_version {:}?", info.protocol_version);
                debug!("agent_version {:?}", info.agent_version);
                debug!("listening_ addresses {:?}", info.listen_addrs);
                debug!("observed_address {:?}", observed_addr);
                debug!("protocols {:?}", info.protocols);
            }
            IdentifyEvent::Sent { .. } => (),
            IdentifyEvent::Error { .. } => (),
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_addrs) => {
                for (peer, _addr) in discovered_addrs {
                    self.events.push(BehaviourEvent::MdnsDiscoveredPeer(peer))
                }
            }
            MdnsEvent::Expired(expired_addrs) => {
                for (peer, _addr) in expired_addrs {
                    if !self.mdns.has_node(&peer) {
                        self.events.push(BehaviourEvent::MdnsExpiredPeer(peer))
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message(peer_id, message_id, message) => {
                self.events.push(BehaviourEvent::GossipsubMessage {
                    peer_id,
                    message_id,
                    data: message.data,
                    topics: message.topics,
                })
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                self.events
                    .push(BehaviourEvent::GossipsubSubscribed { peer_id, topic });
            }
            GossipsubEvent::Unsubscribed { peer_id, topic } => self
                .events
                .push(BehaviourEvent::GossipsubUnsubscribed { peer_id, topic }),
        }
    }
}

/*
impl NetworkBehaviourEventProcess<RpcEvent> for Behaviour {
    fn inject_event(&mut self, event: RpcEvent) {
        match event {
            RpcEvent::PeerDialed(peer_id) => {
                self.events.push(BehaviourEvent::RpcPeerDialed(peer_id));
            }
            RpcEvent::PeerDisconnected(peer_id) => {
                self.events
                    .push(BehaviourEvent::RpcPeerDisconnected(peer_id));
            }
            RpcEvent::Message(peer_id, message) => self
                .events
                .push(BehaviourEvent::RpcMessage(peer_id, message)),
        }
    }
}
*/

impl Behaviour {
    /// Consumes the event list when polled.
    fn poll<TBehaviourIn>(
        &mut self,
        _cx: &mut Context<'_>,
        _params: &mut impl PollParameters,
    ) -> Poll<NetworkBehaviourAction<TBehaviourIn, BehaviourEvent>> {
        if !self.events.is_empty() {
            return Poll::Ready(NetworkBehaviourAction::GenerateEvent(self.events.remove(0)));
        }

        Poll::Pending
    }
}

impl Behaviour {
    pub fn new(local_key: &identity::Keypair) -> Self {
        let local_peer_id = local_key.public().into_peer_id();
        Self {
            ping: Ping::default(),
            identify: Identify::new("plum/libp2p".into(), "0.0.1".into(), local_key.public()),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
            gossipsub: Gossipsub::new(local_peer_id, GossipsubConfig::default()),
            // rpc: Rpc::default(),
            events: vec![],
        }
    }

    /// Publish gossipsub topic.
    pub fn publish(&mut self, topic: &Topic, data: impl Into<Vec<u8>>) {
        self.gossipsub.publish(topic, data);
    }

    /// Subscribe gossipsub topic.
    pub fn subscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.subscribe(topic)
    }

    /// Unsubscribe gossipsub topic.
    pub fn unsubscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.unsubscribe(topic)
    }

    /*
    /// Sends an RPC message (Request/Response) via the RPC protocol.
    pub fn send_rpc(&mut self, peer_id: PeerId, message: RpcMessage) {
        self.rpc.send_rpc(peer_id, message);
    }*/
}

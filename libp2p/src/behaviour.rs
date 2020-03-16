// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use futures::Async;
use libp2p::core::identity::Keypair;
use libp2p::core::PeerId;
use libp2p::gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageId, Topic, TopicHash};
use libp2p::identify::{Identify, IdentifyEvent};
use libp2p::kad::{record::store::MemoryStore, Kademlia, KademliaEvent};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::ping::{Ping, PingEvent};
use libp2p::swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess};
use libp2p::tokio_io::{AsyncRead, AsyncWrite};
use libp2p::NetworkBehaviour;
use log::debug;

use crate::config::{generate_kad_config, HELLO_TOPIC};
use crate::rpc::{RPCEvent, RPCMessage, RPC};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BehaviourEvent", poll_method = "poll")]
pub struct Behaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub rpc: RPC<TSubstream>,
    pub kad: Kademlia<TSubstream, MemoryStore>,
    pub ping: Ping<TSubstream>,
    pub mdns: Mdns<TSubstream>,
    pub identify: Identify<TSubstream>,
    pub gossipsub: Gossipsub<TSubstream>,
    #[behaviour(ignore)]
    events: Vec<BehaviourEvent>,
}

pub enum BehaviourEvent {
    RPC(PeerId, RPCEvent),
    HelloSubscribed(PeerId),
    DiscoveredPeer(PeerId),
    ExpiredPeer(PeerId),
    GossipMessage {
        id: MessageId,
        source: PeerId,
        topics: Vec<TopicHash>,
        data: Vec<u8>,
    },
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<RPCMessage>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, event: RPCMessage) {
        match event {
            RPCMessage::PeerDialed(_peer_id) => {
                // self.events.push(BehaviourEvent::PeerDialed(peer_id))
            }
            RPCMessage::PeerDisconnected(_peer_id) => {
                // self.events.push(BehaviourEvent::PeerDisconnected(peer_id))
            }
            RPCMessage::RPC(peer_id, rpc_event) => {
                self.events.push(BehaviourEvent::RPC(peer_id, rpc_event))
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<MdnsEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.events.push(BehaviourEvent::DiscoveredPeer(peer))
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.events.push(BehaviourEvent::ExpiredPeer(peer))
                    }
                }
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<KademliaEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, _event: KademliaEvent) {
        // TODO: PeerDiscovered via kad bootstrap.
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<GossipsubEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message(source, id, message) => {
                self.events.push(BehaviourEvent::GossipMessage {
                    id,
                    source,
                    topics: message.topics,
                    data: message.data,
                })
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                if topic == TopicHash::from_raw(HELLO_TOPIC) {
                    self.events.push(BehaviourEvent::HelloSubscribed(peer_id));
                }
            }
            GossipsubEvent::Unsubscribed { .. } => {}
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<PingEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, _event: PingEvent) {}
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<IdentifyEvent>
    for Behaviour<TSubstream>
{
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

impl<TSubstream: AsyncRead + AsyncWrite> Behaviour<TSubstream> {
    /// Consume the libp2p event when polled.
    fn poll<TBehaviourIn>(
        &mut self,
    ) -> Async<NetworkBehaviourAction<TBehaviourIn, BehaviourEvent>> {
        if !self.events.is_empty() {
            return Async::Ready(NetworkBehaviourAction::GenerateEvent(self.events.remove(0)));
        }
        Async::NotReady
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> Behaviour<TSubstream> {
    pub fn new(local_key: &Keypair) -> Self {
        let local_peer_id = local_key.public().into_peer_id();
        let (kad_cfg, kad_store) = generate_kad_config(&local_peer_id);
        Self {
            rpc: RPC::new(),
            kad: Kademlia::with_config(local_peer_id.clone(), kad_store, kad_cfg),
            ping: Ping::default(),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
            events: vec![],
            identify: Identify::new("plum/libp2p".into(), "0.0.1".into(), local_key.public()),
            gossipsub: Gossipsub::new(local_peer_id, GossipsubConfig::default()),
        }
    }

    /// Sends an RPC Request/Response via the RPC protocol.
    pub fn send_rpc(&mut self, peer_id: PeerId, rpc_event: RPCEvent) {
        self.rpc.send_rpc(peer_id, rpc_event);
    }

    /// Publish gossipsub topic.
    pub fn publish(&mut self, topic: &Topic, data: impl Into<Vec<u8>>) {
        self.gossipsub.publish(topic, data);
    }

    /// Subscribe gossipsub topic.
    pub fn subscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.subscribe(topic)
    }
}

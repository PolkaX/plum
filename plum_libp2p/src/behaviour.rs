use futures::Async;
use libp2p::core::identity::Keypair;
use libp2p::core::PeerId;
use libp2p::gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, Topic, TopicHash};
use libp2p::identify::{Identify, IdentifyEvent};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::ping::{
    handler::{PingFailure, PingSuccess},
    Ping, PingEvent,
};
use libp2p::swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess};
use libp2p::tokio_io::{AsyncRead, AsyncWrite};
use libp2p::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BehaviourEvent", poll_method = "poll")]
pub struct Behaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub gossipsub: Gossipsub<TSubstream>,
    pub mdns: Mdns<TSubstream>,
    pub ping: Ping<TSubstream>,
    pub identify: Identify<TSubstream>,
    #[behaviour(ignore)]
    events: Vec<BehaviourEvent>,
}

pub enum BehaviourEvent {
    Hello(PeerId),
    DiscoveredPeer(PeerId),
    ExpiredPeer(PeerId),
    GossipMessage {
        source: PeerId,
        topics: Vec<TopicHash>,
        message: Vec<u8>,
    },
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<MdnsEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    println!("-----------discovered peer: {:?}", peer);
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

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<GossipsubEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message(_, _, message) => {
                self.events.push(BehaviourEvent::GossipMessage {
                    source: message.source,
                    topics: message.topics,
                    message: message.data,
                })
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                if topic == TopicHash::from_raw("/fil/hello") {
                    self.events.push(BehaviourEvent::Hello(peer_id));
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
                log::debug!("Identified Peer {:?}", peer_id);
                log::debug!("protocol_version {:}?", info.protocol_version);
                log::debug!("agent_version {:?}", info.agent_version);
                log::debug!("listening_ addresses {:?}", info.listen_addrs);
                log::debug!("observed_address {:?}", observed_addr);
                log::debug!("protocols {:?}", info.protocols);
            }
            IdentifyEvent::Sent { .. } => (),
            IdentifyEvent::Error { .. } => (),
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> Behaviour<TSubstream> {
    /// Consumes the events list when polled.
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
        let gossipsub_config = GossipsubConfig::default();
        Self {
            gossipsub: Gossipsub::new(local_peer_id, gossipsub_config),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
            ping: Ping::default(),
            identify: Identify::new("forest/libp2p".into(), "0.0.1".into(), local_key.public()),
            events: vec![],
        }
    }

    pub fn publish(&mut self, topic: &Topic, data: impl Into<Vec<u8>>) {
        self.gossipsub.publish(topic, data);
    }

    pub fn subscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.subscribe(topic)
    }
}

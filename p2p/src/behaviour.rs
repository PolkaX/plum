// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashSet;
use std::task::{Context, Poll};

use libp2p::{
    core::{identity::Keypair, PeerId},
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, Topic, TopicHash},
    identify::{Identify, IdentifyEvent},
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingEvent, PingFailure, PingSuccess},
    request_response::{
        ProtocolSupport, RequestId, RequestResponse, RequestResponseConfig, RequestResponseEvent,
        RequestResponseMessage, ResponseChannel,
    },
    swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess, PollParameters},
    NetworkBehaviour,
};

use crate::config::Libp2pConfig;
use crate::protocol::{BlockSyncCodec, BlockSyncProtocolName, BlockSyncRequest, BlockSyncResponse};
use crate::protocol::{HelloCodec, HelloProtocolName, HelloRequest, HelloResponse};

/// The behaviour for the network. Allows customizing the swarm.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BehaviourEvent", poll_method = "poll")]
pub struct Behaviour {
    ping: Ping,
    identify: Identify,
    mdns: Mdns,
    kademlia: Kademlia<MemoryStore>,
    gossipsub: Gossipsub,
    hello: RequestResponse<HelloCodec>,
    blocksync: RequestResponse<BlockSyncCodec>,
    #[behaviour(ignore)]
    events: Vec<BehaviourEvent>,
    #[behaviour(ignore)]
    peers: HashSet<PeerId>,
}

/// Event that can happen on the behaviour.
#[doc(hidden)]
pub enum BehaviourEvent {
    MdnsDiscoveredPeer(PeerId),
    MdnsExpiredPeer(PeerId),
    GossipsubMessage {
        source: PeerId,
        data: Vec<u8>,
        topics: Vec<TopicHash>,
    },
    GossipsubSubscribed {
        peer_id: PeerId,
        topic: TopicHash,
    },
    GossipsubUnsubscribed {
        peer_id: PeerId,
        topic: TopicHash,
    },
    HelloRequest {
        peer: PeerId,
        request: HelloRequest,
        channel: ResponseChannel<HelloResponse>,
    },
    HelloResponse {
        peer: PeerId,
        request_id: RequestId,
        response: HelloResponse,
    },
    BlockSyncRequest {
        peer: PeerId,
        request: BlockSyncRequest,
        channel: ResponseChannel<BlockSyncResponse>,
    },
    BlockSyncResponse {
        peer: PeerId,
        request_id: RequestId,
        response: BlockSyncResponse,
    },
}

impl NetworkBehaviourEventProcess<PingEvent> for Behaviour {
    fn inject_event(&mut self, event: PingEvent) {
        match event.result {
            Ok(PingSuccess::Ping { rtt }) => {
                debug!(
                    "[ping] PingSuccess::Ping rtt to {} is {} ms",
                    event.peer,
                    rtt.as_millis()
                );
            }
            Ok(PingSuccess::Pong) => {
                debug!("[ping] PingSuccess::Pong from peer: {}", event.peer);
            }
            Err(PingFailure::Timeout) => {
                debug!("[ping] PingFailure::Timeout from peer: {}", event.peer);
            }
            Err(PingFailure::Other { error }) => {
                debug!("[ping] PingFailure::Other from {}: {}", event.peer, error);
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
                debug!("[identify] Identified Peer {}", peer_id);
                debug!("[identify] protocol_version {}", info.protocol_version);
                debug!("[identify] agent_version {}", info.agent_version);
                debug!("[identify] listening_ addresses {:?}", info.listen_addrs);
                debug!("[identify] observed_address {:?}", observed_addr);
                debug!("[identify] protocols {:?}", info.protocols);
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
                for (peer_id, _addr) in discovered_addrs {
                    debug!("[mdns] Discovered (peer: {})", peer_id);
                    self.peers.insert(peer_id);
                }
            }
            MdnsEvent::Expired(expired_addrs) => {
                for (peer_id, _addr) in expired_addrs {
                    if !self.mdns.has_node(&peer_id) {
                        debug!("[mdns] Expired (peer: {})", peer_id);
                        self.peers.remove(&peer_id);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::RoutingUpdated { peer, .. } => {
                debug!("[kad] RoutingUpdated (peer: {})", peer);
                self.peers.insert(peer);
            }
            event => debug!("[kad] {:?}", event),
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message(peer_id, message_id, message) => {
                debug!(
                    "[gossipsub] Message (peer: {}, message_id: {:?}): {:?}",
                    peer_id, message_id, message
                );
                self.events.push(BehaviourEvent::GossipsubMessage {
                    source: message.source,
                    data: message.data,
                    topics: message.topics,
                })
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                debug!(
                    "[gossipsub] Subscribed topic (peer: {}): {}",
                    peer_id, topic
                );
                self.events
                    .push(BehaviourEvent::GossipsubSubscribed { peer_id, topic });
            }
            GossipsubEvent::Unsubscribed { peer_id, topic } => {
                debug!(
                    "[gossipsub] Unsubscribed topic (peer: {}): {}",
                    peer_id, topic
                );
                self.events
                    .push(BehaviourEvent::GossipsubUnsubscribed { peer_id, topic });
            }
        }
    }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<HelloRequest, HelloResponse>> for Behaviour {
    fn inject_event(&mut self, event: RequestResponseEvent<HelloRequest, HelloResponse>) {
        match event {
            RequestResponseEvent::Message { peer, message } => match message {
                RequestResponseMessage::Request { request, channel } => {
                    debug!(
                        "[request-response] hello request (peer: {}): {:?}",
                        peer, request
                    );
                    self.events.push(BehaviourEvent::HelloRequest {
                        peer,
                        request,
                        channel,
                    });
                }
                RequestResponseMessage::Response {
                    request_id,
                    response,
                } => {
                    debug!(
                        "[request-response] hello response (peer: {}, request_id: {:?}): {:?}",
                        peer, request_id, response
                    );
                    self.events.push(BehaviourEvent::HelloResponse {
                        peer,
                        request_id,
                        response,
                    });
                }
            },
            RequestResponseEvent::OutboundFailure {
                peer,
                request_id,
                error,
            } => {
                warn!(
                    "[request-response] hello outbound failure (peer: {}, request id: {:?}): {:?}",
                    peer, request_id, error
                );
            }
            RequestResponseEvent::InboundFailure { peer, error } => {
                warn!(
                    "[request-response] hello inbound failure (peer: {}): {:?}",
                    peer, error
                );
            }
        }
    }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<BlockSyncRequest, BlockSyncResponse>>
    for Behaviour
{
    fn inject_event(&mut self, event: RequestResponseEvent<BlockSyncRequest, BlockSyncResponse>) {
        match event {
            RequestResponseEvent::Message { peer, message } => match message {
                RequestResponseMessage::Request { request, channel } => {
                    debug!(
                        "[request-response] blocksync request (peer: {}): {:?}",
                        peer, request
                    );
                    self.events.push(BehaviourEvent::BlockSyncRequest {
                        peer,
                        request,
                        channel,
                    });
                }
                RequestResponseMessage::Response {
                    request_id,
                    response,
                } => {
                    debug!(
                        "[request-response] blocksync response (peer: {}, request_id: {:?}): {:?}",
                        peer, request_id, response
                    );
                    self.events.push(BehaviourEvent::BlockSyncResponse {
                        peer,
                        request_id,
                        response,
                    });
                }
            },
            RequestResponseEvent::OutboundFailure {
                peer,
                request_id,
                error,
            } => {
                warn!(
                    "[request-response] blocksync outbound failure (peer: {}, request id: {:?}): {:?}",
                    peer, request_id, error
                );
            }
            RequestResponseEvent::InboundFailure { peer, error } => {
                warn!(
                    "[request-response] blocksync inbound failure (peer: {}): {:?}",
                    peer, error
                );
            }
        }
    }
}

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
    /// Create a new network behaviour.
    pub fn new(local_key_pair: &Keypair, config: &Libp2pConfig, network_name: &str) -> Self {
        let local_public = local_key_pair.public();
        let local_peer_id = local_public.clone().into_peer_id();

        // Create the Kademlia DHT service with the bootnodes from the config.
        let mut kademlia = config.build_kademlia(local_peer_id.clone(), network_name);
        if let Err(err) = kademlia.bootstrap() {
            warn!("Kademlia bootstrap error: {}", err);
        }

        // Create hello request-response service.
        let hello = RequestResponse::new(
            HelloCodec,
            vec![(HelloProtocolName, ProtocolSupport::Full)],
            RequestResponseConfig::default(),
        );

        // Create blocksync request-response service.
        let blocksync = RequestResponse::new(
            BlockSyncCodec,
            vec![(BlockSyncProtocolName, ProtocolSupport::Full)],
            RequestResponseConfig::default(),
        );

        Self {
            ping: Ping::default(),
            identify: Identify::new("ipfs/0.1.0".into(), "plum/0.1.0".into(), local_public),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
            kademlia,
            gossipsub: Gossipsub::new(local_peer_id, GossipsubConfig::default()),
            hello,
            blocksync,
            events: vec![],
            peers: HashSet::default(),
        }
    }

    /// Publish message to the network over gossipsub protocol.
    pub fn publish(&mut self, topic: &Topic, data: impl Into<Vec<u8>>) {
        self.gossipsub.publish(topic, data);
    }

    /// Subscribe to a topic.
    pub fn subscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.subscribe(topic)
    }

    /// Unsubscribe from a topic.
    pub fn unsubscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.unsubscribe(topic)
    }

    /// Initiates sending a hello request.
    pub fn send_hello_request(&mut self, peer: &PeerId, request: HelloRequest) -> RequestId {
        self.hello.send_request(peer, request)
    }

    /// Initiates sending a blocksync request.
    pub fn send_blocksync_request(
        &mut self,
        peer: &PeerId,
        request: BlockSyncRequest,
    ) -> RequestId {
        self.blocksync.send_request(peer, request)
    }

    /// Return the peer set.
    pub fn peers(&self) -> &HashSet<PeerId> {
        &self.peers
    }
}

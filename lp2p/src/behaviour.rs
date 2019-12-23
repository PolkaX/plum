// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;
use std::time::Instant;

use futures::sync::mpsc::UnboundedSender;
use libp2p::core::{either::EitherOutput, ConnectedPoint};
use libp2p::swarm::{IntoProtocolsHandler, IntoProtocolsHandlerSelect, ProtocolsHandler};
use libp2p::swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters};
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, FloodsubMessage, Topic},
    kad::{record::store::MemoryStore, Kademlia},
    tokio_io::{AsyncRead, AsyncWrite},
    Multiaddr, PeerId,
};
use log::info;
use tokio::prelude::Async;

use crate::config;

#[derive(Debug)]
pub enum PeerState {
    /// The peer misbehaved. If the PSM wants us to connect to this node, we will add an artificial
    /// delay to the connection.
    Banned {
        /// Until when the node is banned.
        until: Instant,
    },

    /// We are connected to this peer and the peerset has accepted it. The handler is in the
    /// enabled state.
    Enabled {
        /// How we are connected to this peer.
        connected_point: ConnectedPoint,
        /// If true, we have a custom protocol open with this peer.
        open: bool,
    },
}

// We create a custom network behaviour that combines floodsub and kad.
// In the future, we want to improve libp2p to make this easier to do.
pub struct Behaviour<TSubstream> {
    pub floodsub: Floodsub<TSubstream>,
    pub kad: Kademlia<TSubstream, MemoryStore>,
    pub sender: UnboundedSender<Event>,
    pub peers: HashMap<PeerId, PeerState>,
}

#[derive(Debug)]
pub enum Msg {
    Hello(HelloMsg),
    FIL,
}

impl Msg {
    pub fn to_vec(self) -> Vec<u8> {
        b"encode the message".to_vec()
    }
}

#[derive(Debug)]
pub enum Event {
    Connecting(PeerId),
    Message(FloodsubMessage),
}

#[derive(Debug)]
pub struct HelloMsg {
    pub peer_id: PeerId,
}

impl<TSubstream> Behaviour<TSubstream> {
    pub fn new(local_peer_id: &PeerId, sender: UnboundedSender<Event>) -> Self {
        let (cfg, store) = config::configure_kad(local_peer_id);
        let _cid = config::configure_genesis_hash();

        Behaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            kad: Kademlia::with_config(local_peer_id.clone(), store, cfg),
            sender,
            peers: HashMap::new(),
        }
    }

    pub fn send(&mut self, topic: Topic, _msg: &Msg) {
        // encode msg to Vec<u8>
        let mut data = Vec::<u8>::new();
        data.push(2);
        self.floodsub.publish(topic, data);
    }

    pub fn on_event(&mut self, event: Event) {
        let _ = self.sender.unbounded_send(event);
    }

    pub fn on_connet(&mut self) {
        // self.floodsub.publish(topic, b"123".to_vec());
    }
}

impl<TSubstream> NetworkBehaviour for Behaviour<TSubstream>
where
    TSubstream: AsyncRead + AsyncWrite,
{
    type ProtocolsHandler = IntoProtocolsHandlerSelect<
        <Floodsub<TSubstream> as NetworkBehaviour>::ProtocolsHandler,
        <Kademlia<TSubstream, MemoryStore> as NetworkBehaviour>::ProtocolsHandler,
    >;
    type OutEvent = Msg;
    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        IntoProtocolsHandler::select(self.floodsub.new_handler(), self.kad.new_handler())
    }

    fn addresses_of_peer(&mut self, _peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: PeerId, endpoint: ConnectedPoint) {
        self.floodsub
            .inject_connected(peer_id.clone(), endpoint.clone());
        self.kad.inject_connected(peer_id.clone(), endpoint.clone());
        info!("inject_connected, peer_id:{:?}", peer_id.clone());
        self.floodsub.add_node_to_partial_view(peer_id.clone());
        self.peers.insert(
            peer_id,
            PeerState::Enabled {
                connected_point: endpoint,
                open: true,
            },
        );
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId, endpoint: ConnectedPoint) {
        self.floodsub.inject_disconnected(peer_id, endpoint.clone());
        self.kad.inject_disconnected(peer_id, endpoint);
        self.peers.remove(peer_id);
    }

    fn inject_replaced(
        &mut self,
        peer_id: PeerId,
        closed_endpoint: ConnectedPoint,
        new_endpoint: ConnectedPoint,
    ) {
        self.floodsub.inject_replaced(
            peer_id.clone(),
            closed_endpoint.clone(),
            new_endpoint.clone(),
        );
        self.kad
            .inject_replaced(peer_id, closed_endpoint, new_endpoint);
    }

    fn inject_node_event(
        &mut self,
        peer_id: PeerId,
        event: <<Self::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::OutEvent,
    ) {
        info!("inject_node_event");
        match event {
            EitherOutput::First(event) => {
                self.floodsub.inject_node_event(peer_id, event);
            }
            EitherOutput::Second(event) => self.kad.inject_node_event(peer_id, event),
        }
    }

    fn inject_addr_reach_failure(
        &mut self,
        peer_id: Option<&PeerId>,
        addr: &Multiaddr,
        error: &dyn std::error::Error,
    ) {
        self.floodsub
            .inject_addr_reach_failure(peer_id, addr, error);
        self.kad.inject_addr_reach_failure(peer_id, addr, error);
    }

    fn inject_dial_failure(&mut self, peer_id: &PeerId) {
        self.floodsub.inject_dial_failure(peer_id);
        self.kad.inject_dial_failure(peer_id);
    }

    fn inject_new_listen_addr(&mut self, addr: &Multiaddr) {
        self.floodsub.inject_new_listen_addr(addr);
        self.kad.inject_new_listen_addr(addr);
    }

    fn inject_expired_listen_addr(&mut self, addr: &Multiaddr) {
        self.floodsub.inject_expired_listen_addr(addr);
        self.kad.inject_expired_listen_addr(addr);
    }

    fn inject_new_external_addr(&mut self, addr: &Multiaddr) {
        self.floodsub.inject_new_external_addr(addr);
        self.kad.inject_new_external_addr(addr);
    }

    fn poll(
        &mut self,
        params: &mut impl PollParameters
    ) -> Async<
        NetworkBehaviourAction<
            <<Self::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::InEvent,
            Self::OutEvent
        >
>{
        info!("poll");

        loop {
            match self.floodsub.poll(params) {
                Async::NotReady => break,
                Async::Ready(NetworkBehaviourAction::GenerateEvent(ev)) => {
                    info!("floodsub poll");
                    match ev {
                        FloodsubEvent::Message(msg) => {
                            self.on_event(Event::Message(msg));
                        }
                        FloodsubEvent::Subscribed { peer_id, .. } => {
                            self.on_event(Event::Connecting(peer_id));
                        }
                        FloodsubEvent::Unsubscribed { .. } => {}
                    }
                }
                Async::Ready(NetworkBehaviourAction::DialAddress { address }) => {
                    return Async::Ready(NetworkBehaviourAction::DialAddress { address })
                }
                Async::Ready(NetworkBehaviourAction::DialPeer { peer_id }) => {
                    return Async::Ready(NetworkBehaviourAction::DialPeer { peer_id })
                }
                Async::Ready(NetworkBehaviourAction::SendEvent { peer_id, event }) => {
                    info!("floodsub poll send event");
                    return Async::Ready(NetworkBehaviourAction::SendEvent {
                        peer_id,
                        event: EitherOutput::First(event),
                    });
                }
                Async::Ready(NetworkBehaviourAction::ReportObservedAddr { address }) => {
                    return Async::Ready(NetworkBehaviourAction::ReportObservedAddr { address })
                }
            }
        }

        loop {
            match self.kad.poll(params) {
                Async::NotReady => break,
                Async::Ready(NetworkBehaviourAction::GenerateEvent(_ev)) => {
                    info!("kad poll");
                    //return NetworkBehaviourAction::GenerateEvent(ev);
                }
                Async::Ready(NetworkBehaviourAction::DialAddress { address }) => {
                    return Async::Ready(NetworkBehaviourAction::DialAddress { address })
                }
                Async::Ready(NetworkBehaviourAction::DialPeer { peer_id }) => {
                    return Async::Ready(NetworkBehaviourAction::DialPeer { peer_id })
                }
                Async::Ready(NetworkBehaviourAction::SendEvent { peer_id, event }) => {
                    return Async::Ready(NetworkBehaviourAction::SendEvent {
                        peer_id,
                        event: EitherOutput::Second(event),
                    })
                }
                Async::Ready(NetworkBehaviourAction::ReportObservedAddr { address }) => {
                    return Async::Ready(NetworkBehaviourAction::ReportObservedAddr { address })
                }
            }
        }

        Async::NotReady
    }
}

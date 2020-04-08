// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::task::{Context, Poll};

use libp2p::{
    core::{connection::ConnectionId, Multiaddr, PeerId},
    swarm::{
        protocols_handler::ProtocolsHandler, NetworkBehaviour, NetworkBehaviourAction,
        PollParameters,
    },
};

use crate::new_rpc::handler::RpcHandler;
use crate::new_rpc::protocol::RpcMessage;

/// Messages sent to the user from the RPC protocol.
#[derive(Debug)]
pub enum RpcEvent {
    PeerDialed(PeerId),
    PeerDisconnected(PeerId),
    Message(PeerId, RpcMessage),
}

/// The RPC behaviour that gets consumed by the Swarm.
#[derive(Default)]
pub struct Rpc {
    /// Queue of events to processed.
    events: Vec<NetworkBehaviourAction<RpcMessage, RpcEvent>>,
}

impl Rpc {
    /// Creates a new RPC behaviour
    pub fn new() -> Self {
        Rpc::default()
    }

    /// Send an RPC message to a peer specified by peer_id.
    pub fn send_rpc(&mut self, peer_id: PeerId, msg: RpcMessage) {
        self.events
            .push(NetworkBehaviourAction::GenerateEvent(RpcEvent::Message(
                peer_id, msg,
            )));
    }
}

impl NetworkBehaviour for Rpc {
    type ProtocolsHandler = RpcHandler;
    type OutEvent = RpcEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        RpcHandler::default()
    }

    fn addresses_of_peer(&mut self, _peer_id: &PeerId) -> Vec<Multiaddr> {
        vec![]
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {
        info!("New peer connected: {:?}", peer_id);
        self.events
            .push(NetworkBehaviourAction::GenerateEvent(RpcEvent::PeerDialed(
                peer_id.clone(),
            )))
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        info!("Peer disconnected: {:?}", peer_id);
        self.events.push(NetworkBehaviourAction::GenerateEvent(
            RpcEvent::PeerDisconnected(peer_id.clone()),
        ))
    }

    fn inject_event(
        &mut self,
        peer_id: PeerId,
        _connection: ConnectionId,
        event: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
        self.events
            .push(NetworkBehaviourAction::GenerateEvent(RpcEvent::Message(
                peer_id, event,
            )))
    }

    fn poll(
        &mut self,
        _cx: &mut Context<'_>,
        _params: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if !self.events.is_empty() {
            return Poll::Ready(self.events.remove(0));
        }
        Poll::Pending
    }
}

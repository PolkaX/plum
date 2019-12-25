// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use std::{collections::HashMap, pin::Pin, task::Context, task::Poll, time::Instant};

use futures::{channel::mpsc, prelude::*};
use libp2p::{Multiaddr, PeerId};
use log::info;

const MAX_FIL_PEERS: u32 = 32;
const MIN_FIL_PEERS: u32 = 8;

#[derive(Debug)]
pub enum Action {
    AddPeer(PeerId),
    RemovePeer(PeerId),
}

/// Shared handle to the peer set manager (PSM). Distributed around the code.
#[derive(Debug, Clone)]
pub struct PeerMgrHandle {
    tx: mpsc::UnboundedSender<Action>,
}

impl PeerMgrHandle {
    pub fn add_peer(&self, peer_id: PeerId) {
        let _ = self.tx.unbounded_send(Action::AddPeer(peer_id));
    }

    pub fn remove_peer(&self, peer_id: PeerId) {
        let _ = self.tx.unbounded_send(Action::RemovePeer(peer_id));
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Enabled,
    NotConnected,
}

#[derive(Debug)]
pub struct PeerMgr {
    bootstrappers: Vec<Multiaddr>,
    peers: HashMap<PeerId, ConnectionState>,
    /// Receiver for messages from the `PeersetHandle` and from `tx`.
    rx: mpsc::UnboundedReceiver<Action>,
    /// Sending side of `rx`.
    tx: mpsc::UnboundedSender<Action>,
    /// Queue of messages to be emitted when the `Peerset` is polled.
    max_fil_peers: u32,
    min_fil_peers: u32,
    expanding: bool,
    created: Instant,
}

impl PeerMgr {
    pub fn new() -> (Self, PeerMgrHandle) {
        let (tx, rx) = mpsc::unbounded();

        let peermgr = Self {
            tx: tx.clone(),
            rx,
            created: Instant::now(),
            bootstrappers: Vec::new(),
            peers: Default::default(),
            max_fil_peers: MAX_FIL_PEERS,
            min_fil_peers: MIN_FIL_PEERS,
            expanding: false,
        };

        let handle = PeerMgrHandle { tx };

        (peermgr, handle)
    }

    pub fn on_add_peer(&mut self, peer_id: PeerId) {
        // TODO check max peers, ignore the incoming if the max reached?
        info!("add {:?} to peer manager", peer_id);
        self.peers.insert(peer_id, ConnectionState::Enabled);
    }

    pub fn on_remove_peer(&mut self, peer_id: &PeerId) {
        // TODO check min peers and do expand if neccessary.
        info!("[peermgr] remove {:?} from peer manager", peer_id);
        self.peers.remove(peer_id);
    }
}

impl Stream for PeerMgr {
    type Item = Action;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            let action = match Stream::poll_next(Pin::new(&mut self.rx), cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(event)) => event,
                Poll::Ready(None) => return Poll::Pending,
            };

            match action {
                Action::AddPeer(peer_id) => self.on_add_peer(peer_id),
                Action::RemovePeer(peer_id) => self.on_remove_peer(&peer_id),
            }
        }
    }
}

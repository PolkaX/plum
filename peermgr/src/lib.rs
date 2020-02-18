// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;
use std::time::Instant;

use futures::channel::mpsc;
use libp2p_core::{Multiaddr, PeerId};
use log::{debug, warn};

pub const MAX_FIL_PEERS: u32 = 32;
pub const MIN_FIL_PEERS: u32 = 8;

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
    pub peers: HashMap<PeerId, ConnectionState>,
    /// Receiver for messages from the `PeerMgrHandle` and from `tx`.
    pub rx: mpsc::UnboundedReceiver<Action>,
    /// Sending side of `rx`.
    tx: mpsc::UnboundedSender<Action>,
    pub max_fil_peers: u32,
    pub min_fil_peers: u32,
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

    pub fn get_peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn on_add_peer(&mut self, peer_id: PeerId) {
        if self.get_peer_count() < self.max_fil_peers as usize {
            debug!(target: "peermgr", "[on_add_peer] a new peer added: {}", peer_id);
            self.peers.insert(peer_id, ConnectionState::Enabled);
        } else {
            debug!(
                target: "peermgr",
                "[on_add_peer] max_fil_peers reached, new peer {} ignored",
                peer_id
            );
        }
    }

    pub fn on_remove_peer(&mut self, peer_id: &PeerId) {
        // TODO check min peers and do expand if neccessary.
        self.peers.remove(peer_id);
        if self.get_peer_count() < self.min_fil_peers as usize {
            warn!(
                target: "peermgr",
                "[on_remove_peer] current peer count {:?} is less than the expected min_fil_peers: {:?}",
                self.get_peer_count(),
                self.min_fil_peers
            );
        }
    }
}

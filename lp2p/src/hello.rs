// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

// use cid::Cid;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

type Cid = u8;

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerIdBytes(pub Vec<u8>);

impl From<PeerId> for PeerIdBytes {
    fn from(peer_id: PeerId) -> Self {
        Self(peer_id.into_bytes())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub heaviest_tip_set: Cid,
    pub heaviest_tip_set_weight: u128,
    pub genesis_hash: Cid,
    pub sender: PeerIdBytes,
}

impl Message {
    pub fn new(
        heaviest_tip_set: Cid,
        heaviest_tip_set_weight: u128,
        genesis_hash: Cid,
        sender: PeerIdBytes,
    ) -> Self {
        Self {
            heaviest_tip_set,
            heaviest_tip_set_weight,
            genesis_hash,
            sender,
        }
    }
}

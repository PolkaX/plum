// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec, IntoExt};
use libp2p::{
    core::{Multiaddr, PeerId},
    gossipsub::Topic,
    kad::{record::store::MemoryStore, KademliaConfig},
};

///
#[derive(Debug)]
pub struct Libp2pConfig {
    pub listen_address: Multiaddr,
    pub bootnodes: Vec<Multiaddr>,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        Self {
            listen_address: "/ip4/0.0.0.0/tcp/0".parse().unwrap(),
            bootnodes: vec![],
        }
    }
}

///
pub fn genesis_hash() -> Cid {
    let hash = multihash::Sha2_256::digest(GENESIS).into_ext();
    Cid::new_v1(Codec::DagProtobuf, hash)
}

///
pub fn kad_config(peer_id: &PeerId) -> (KademliaConfig, MemoryStore) {
    let mut cfg = KademliaConfig::default();
    cfg.set_query_timeout(std::time::Duration::from_secs(5 * 60));
    let store = MemoryStore::new(peer_id.clone());
    (cfg, store)
}

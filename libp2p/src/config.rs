// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec, IntoExt};
use libp2p::gossipsub::Topic;
use libp2p::kad::{record::store::MemoryStore, KademliaConfig};
use libp2p::{Multiaddr, PeerId};

pub const GENESIS: &[u8] = b"filecoin plum";

pub const HELLO_TOPIC: &str = "/fil/hello";
pub const BLOCKS_TOPIC: &str = "/fil/blocks";
pub const MESSAGES_TOPIC: &str = "/fil/messages";

#[derive(Debug)]
pub struct Libp2pConfig {
    pub listen_address: Multiaddr,
    pub bootnodes: Vec<Multiaddr>,
    pub pubsub_topics: Vec<Topic>,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        Self {
            listen_address: "/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>().unwrap(),
            bootnodes: vec![],
            pubsub_topics: vec![
                Topic::new(HELLO_TOPIC.into()),
                Topic::new(BLOCKS_TOPIC.into()),
                Topic::new(MESSAGES_TOPIC.into()),
            ],
        }
    }
}

pub fn genesis_hash() -> Cid {
    let hash = multihash::Sha2_256::digest(GENESIS).into_ext();
    Cid::new_v1(Codec::DagProtobuf, hash)
}

pub fn generate_kad_config(peer_id: &PeerId) -> (KademliaConfig, MemoryStore) {
    let mut cfg = KademliaConfig::default();
    cfg.set_query_timeout(std::time::Duration::from_secs(5 * 60));
    let store = MemoryStore::new(peer_id.clone());
    (cfg, store)
}

// Copyright 2019 PolkaX. Licensed under GPL-3.0.

use cid::{Cid, Codec, Version};
use libp2p::{
    floodsub::{Topic, TopicBuilder},
    identity,
    kad::{record::store::MemoryStore, KademliaConfig},
    PeerId,
};

pub fn configure_key() -> (identity::Keypair, PeerId) {
    // Create a random PeerId
    let local_key = identity::Keypair::generate_secp256k1();
    let local_peer_id = PeerId::from(local_key.public());
    (local_key, local_peer_id)
}

pub fn hello_topic() -> Topic {
    TopicBuilder::new("/fil/hello").build()
}

pub fn msg_topic() -> Topic {
    TopicBuilder::new("/fil/messages").build()
}

pub fn blocks_topic() -> Topic {
    TopicBuilder::new("/fil/blocks").build()
}

pub fn configure_kad(peer_id: &PeerId) -> (KademliaConfig, MemoryStore) {
    let mut cfg = KademliaConfig::default();
    cfg.set_query_timeout(std::time::Duration::from_secs(5 * 60));
    let store = MemoryStore::new(peer_id.clone());
    (cfg, store)
}

pub fn configure_genesis_hash() -> Cid {
    let h = multihash::encode(multihash::Hash::SHA2256, b"filecoin plum").unwrap();
    Cid::new(Codec::DagProtobuf, Version::V1, &h)
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use libp2p::gossipsub::Topic;
use libp2p::Multiaddr;

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

use libp2p::gossipsub::Topic;
use libp2p::Multiaddr;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Libp2pConfig {
    pub listening_multiaddr: String,
    pub bootnodes: Vec<Multiaddr>,

    #[serde(skip_deserializing)] // Always use default
    pub pubsub_topics: Vec<Topic>,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        Self {
            listening_multiaddr: "/ip4/0.0.0.0/tcp/0".to_owned(),
            pubsub_topics: vec![
                Topic::new("/fil/hello".to_owned()),
                Topic::new("/fil/blocks".to_owned()),
                Topic::new("/fil/messages".to_owned()),
            ],
            bootnodes: vec![],
        }
    }
}

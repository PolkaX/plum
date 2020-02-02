use libp2p::gossipsub::Topic;
use libp2p::Multiaddr;

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
                Topic::new("/fil/hello".to_owned()),
                Topic::new("/fil/blocks".to_owned()),
                Topic::new("/fil/messages".to_owned()),
            ],
        }
    }
}

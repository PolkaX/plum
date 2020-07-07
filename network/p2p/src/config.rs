// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::time::Duration;

use libp2p::{
    core::{Multiaddr, PeerId},
    gossipsub::Topic,
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig},
    multiaddr::Protocol,
};

// See lotus/build/bootstrap/bootstrappers.pi
const BOOTSTRAP_NODES: &[&str] = &[
    "/dns4/bootstrap-0-sin.fil-test.net/tcp/1347/p2p/12D3KooWPdUquftaQvoQEtEdsRBAhwD6jopbF2oweVTzR59VbHEd",
    "/ip4/86.109.15.57/tcp/1347/p2p/12D3KooWPdUquftaQvoQEtEdsRBAhwD6jopbF2oweVTzR59VbHEd",
    "/dns4/bootstrap-0-dfw.fil-test.net/tcp/1347/p2p/12D3KooWQSCkHCzosEyrh8FgYfLejKgEPM5VB6qWzZE3yDAuXn8d",
    "/ip4/139.178.84.45/tcp/1347/p2p/12D3KooWQSCkHCzosEyrh8FgYfLejKgEPM5VB6qWzZE3yDAuXn8d",
    "/dns4/bootstrap-0-fra.fil-test.net/tcp/1347/p2p/12D3KooWEXN2eQmoyqnNjde9PBAQfQLHN67jcEdWU6JougWrgXJK",
    "/ip4/136.144.49.17/tcp/1347/p2p/12D3KooWEXN2eQmoyqnNjde9PBAQfQLHN67jcEdWU6JougWrgXJK",
    "/dns4/bootstrap-1-sin.fil-test.net/tcp/1347/p2p/12D3KooWLmJkZd33mJhjg5RrpJ6NFep9SNLXWc4uVngV4TXKwzYw",
    "/ip4/86.109.15.123/tcp/1347/p2p/12D3KooWLmJkZd33mJhjg5RrpJ6NFep9SNLXWc4uVngV4TXKwzYw",
    "/dns4/bootstrap-1-dfw.fil-test.net/tcp/1347/p2p/12D3KooWGXLHjiz6pTRu7x2pkgTVCoxcCiVxcNLpMnWcJ3JiNEy5",
    "/ip4/139.178.86.3/tcp/1347/p2p/12D3KooWGXLHjiz6pTRu7x2pkgTVCoxcCiVxcNLpMnWcJ3JiNEy5",
    "/dns4/bootstrap-1-fra.fil-test.net/tcp/1347/p2p/12D3KooW9szZmKttS9A1FafH3Zc2pxKwwmvCWCGKkRP4KmbhhC4R",
    "/ip4/136.144.49.131/tcp/1347/p2p/12D3KooW9szZmKttS9A1FafH3Zc2pxKwwmvCWCGKkRP4KmbhhC4R",
];

// See https://filecoin-project.github.io/specs/#systems__filecoin_nodes__network for details.
const PUBSUB_TOPICS: &[&str] = &["/fil/blocks", "/fil/msgs"];

/// The config of p2p network.
#[derive(Debug)]
pub struct Libp2pConfig {
    /// The local address for listening.
    pub listen_address: Multiaddr,

    /// The address list of bootstrap nodes.
    pub boot_nodes: Vec<Multiaddr>,

    /// The network name.
    pub network_name: String,

    /// The pubsub topics.
    pub pubsub_topics: Vec<Topic>,
}

impl Default for Libp2pConfig {
    fn default() -> Self {
        let network_name = "lotus";
        Self {
            listen_address: "/ip4/0.0.0.0/tcp/0".parse().unwrap(),
            boot_nodes: BOOTSTRAP_NODES
                .iter()
                .map(|node| node.parse().unwrap())
                .collect(),
            network_name: network_name.into(),
            pubsub_topics: PUBSUB_TOPICS
                .iter()
                .map(|topic| Topic::new(format!("{}/{}", topic, network_name)))
                .collect(),
        }
    }
}

impl Libp2pConfig {
    /// Create a Kademlia DHT.
    pub fn build_kademlia(
        &self,
        local_peer_id: PeerId,
        network_name: &str,
    ) -> Kademlia<MemoryStore> {
        let store = MemoryStore::new(local_peer_id.clone());

        let mut kad_cfg = KademliaConfig::default();
        // see https://filecoin-project.github.io/specs/#systems__filecoin_nodes__network
        kad_cfg.set_protocol_name(format!("fil/{}/kad/1.0.0", network_name).into_bytes());
        kad_cfg.set_query_timeout(Duration::from_secs(5 * 60));

        let mut kad = Kademlia::with_config(local_peer_id, store, kad_cfg);
        for multiaddr in &self.boot_nodes {
            let mut addr = multiaddr.to_owned();
            if let Some(Protocol::P2p(mh)) = addr.pop() {
                let peer_id = PeerId::from_multihash(mh).unwrap();
                kad.add_address(&peer_id, addr);
            } else {
                warn!("Couldn't add address {} to Kademlia DHT", multiaddr);
            }
        }
        kad
    }
}

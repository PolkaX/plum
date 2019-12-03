// Copyright 2019 杭州链网科技

mod hello;

use cid::{Cid, Codec, Version};
use futures::prelude::*;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    swarm::NetworkBehaviourEventProcess,
    tokio_io::{AsyncRead, AsyncWrite},
    NetworkBehaviour, PeerId, Swarm,
};
use std::time::Duration;
use tokio::runtime::TaskExecutor;

// We create a custom network behaviour that combines floodsub and kad.
// In the future, we want to improve libp2p to make this easier to do.
#[derive(NetworkBehaviour)]
pub struct Behaviour<TSubstream: AsyncRead + AsyncWrite> {
    floodsub: Floodsub<TSubstream>,
    kad: Kademlia<TSubstream, MemoryStore>,
    hello: hello::Hello<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<FloodsubEvent>
    for Behaviour<TSubstream>
{
    // Called when `floodsub` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            println!(
                "Received: '{:?}' from {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
            // To Do: handle messages, call back.
            handle_message(message.data);
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<KademliaEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, _event: KademliaEvent) {}
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<hello::HelloEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, _event: hello::HelloEvent) {}
}

fn handle_message(_msg: Vec<u8>) {
    // 1, decode msg
    // 2, handle_event
}

#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    listenning: bool,
}

pub fn initialize(task_executor: TaskExecutor, mut network_state: NetworkState) {
    // Create a random PeerId
    let local_key = identity::Keypair::generate_secp256k1();
    let local_peer_id = PeerId::from(local_key.public());
    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key);

    // Create a Floodsub topic
    let floodsub_topic = floodsub::TopicBuilder::new("/fil/messages").build();

    let mut cfg = KademliaConfig::default();
    cfg.set_query_timeout(Duration::from_secs(5 * 60));
    let store = MemoryStore::new(local_peer_id.clone());

    let h = multihash::encode(multihash::Hash::SHA2256, b"filecoin plum").unwrap();
    let cid = Cid::new(Codec::DagProtobuf, Version::V1, &h);

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut behaviour = Behaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            kad: Kademlia::with_config(local_peer_id.clone(), store, cfg),
            hello: hello::Hello::new(cid),
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::new(transport, behaviour, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    task_executor.spawn(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(_)) => {}
                Async::Ready(None) | Async::NotReady => {
                    if !network_state.listenning {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {:?}", a);
                        }
                        network_state.listenning = true;
                    }
                }
            }
        }
    }));
}

// Copyright 2019 杭州链网科技

use futures::prelude::*;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    tokio_io::{AsyncRead, AsyncWrite},
    NetworkBehaviour, PeerId, Swarm,
};
use tokio::runtime::TaskExecutor;

// We create a custom network behaviour that combines floodsub and mDNS.
// In the future, we want to improve libp2p to make this easier to do.
#[derive(NetworkBehaviour)]
struct MyBehaviour<TSubstream: AsyncRead + AsyncWrite> {
    floodsub: Floodsub<TSubstream>,
    mdns: Mdns<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<MdnsEvent>
    for MyBehaviour<TSubstream>
{
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<FloodsubEvent>
    for MyBehaviour<TSubstream>
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

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
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

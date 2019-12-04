// Copyright 2019 PolkaX

use crate::config;
use crate::hello;
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent},
    swarm::NetworkBehaviourEventProcess,
    tokio_io::{AsyncRead, AsyncWrite},
    NetworkBehaviour, PeerId,
};

// We create a custom network behaviour that combines floodsub and kad.
// In the future, we want to improve libp2p to make this easier to do.
#[derive(NetworkBehaviour)]
pub struct Behaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub floodsub: Floodsub<TSubstream>,
    kad: Kademlia<TSubstream, MemoryStore>,
    hello: hello::Hello<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> Behaviour<TSubstream> {
    pub fn new(local_peer_id: &PeerId) -> Self {
        let (cfg, store) = config::configure_kad(local_peer_id);
        let cid = config::configure_genesis_hash();

        Behaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            kad: Kademlia::with_config(local_peer_id.clone(), store, cfg),
            hello: hello::Hello::new(cid),
        }
    }
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

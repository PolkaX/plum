// Copyright 2019 PolkaX.

use crate::config;
use crate::hello;
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent},
    ping::{Ping, PingConfig, PingEvent},
    swarm::NetworkBehaviourEventProcess,
    tokio_io::{AsyncRead, AsyncWrite},
    NetworkBehaviour, PeerId,
};
use log::info;

// We create a custom network behaviour that combines floodsub and kad.
// In the future, we want to improve libp2p to make this easier to do.
#[derive(NetworkBehaviour)]
pub struct Behaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub floodsub: Floodsub<TSubstream>,
    pub kad: Kademlia<TSubstream, MemoryStore>,
    ping: Ping<TSubstream>,
    hello: hello::Hello<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> Behaviour<TSubstream> {
    pub fn new(local_peer_id: &PeerId) -> Self {
        let (cfg, store) = config::configure_kad(local_peer_id);
        let cid = config::configure_genesis_hash();

        Behaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            kad: Kademlia::with_config(local_peer_id.clone(), store, cfg),
            ping: Ping::new(PingConfig::new().with_keep_alive(true)),
            hello: hello::Hello::new(cid),
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<PingEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, e: PingEvent) {
        info!("rcv ping event: {:?}", e);
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<FloodsubEvent>
    for Behaviour<TSubstream>
{
    // Called when `floodsub` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        info!("rcv floodsub msg:{:?}", message);
        if let FloodsubEvent::Message(message) = message {
            info!(
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
    fn inject_event(&mut self, event: KademliaEvent) {
        info!("rcv kad event: {:?}", event);
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<hello::HelloEvent>
    for Behaviour<TSubstream>
{
    fn inject_event(&mut self, event: hello::HelloEvent) {
        info!("rcv hello event: {:?}", event);
    }
}

fn handle_message(_msg: Vec<u8>) {
    // 1, decode msg
    // 2, handle_event
}

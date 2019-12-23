// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

mod behaviour;
mod config;

use futures::prelude::*;
use futures::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use libp2p::{core::Multiaddr, Swarm};
use log::info;
use tokio::runtime::TaskExecutor;

use crate::behaviour::Event;

#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    listening: bool,
}

pub fn initialize<C: chain::Client>(
    task_executor: TaskExecutor,
    mut network_state: NetworkState,
    peer_ip: Option<Multiaddr>,
    client: C,
) {
    let (local_key, local_peer_id) = config::configure_key();
    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key);

    let (sender, mut receiver): (UnboundedSender<Event>, UnboundedReceiver<Event>) =
        mpsc::unbounded();

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut bh = behaviour::Behaviour::new(&local_peer_id, sender);
        bh.floodsub.subscribe(config::hello_topic());
        Swarm::new(transport, bh, local_peer_id.clone())
    };

    let listen_address: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
    // Listen on all interfaces and whatever port the OS assigns
    Swarm::listen_on(&mut swarm, listen_address.clone()).unwrap();
    swarm.kad.add_address(&local_peer_id, listen_address);
    if let Some(peer_ip) = peer_ip {
        Swarm::dial_addr(&mut swarm, peer_ip);
    }

    task_executor.spawn(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match receiver.poll().expect("Error polling in receiver channel") {
                Async::Ready(None) | Async::NotReady => break,
                Async::Ready(Some(e)) => {
                    info!("mpsc receiver channel received: {:?}", e);
                    match e {
                        Event::Connecting(peer_id) => {
                            info!("---- mpsc receiver channel connecting : {:?}", peer_id);
                            // TODO handle the messages
                            // let best_hash = client.best_hash();
                            let msg = behaviour::Msg::Hello(behaviour::HelloMsg { peer_id });
                            info!("--------- send hello topic");
                            let data = msg.to_vec();
                            swarm.floodsub.publish(config::hello_topic(), data);
                        }
                        Event::Message(_msg) => {
                            // on FloodsubMessage
                        }
                    }
                }
            }
        }

        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(e)) => {
                    info!("rcv event:{:?}", e);
                }
                Async::Ready(None) | Async::NotReady => {
                    if !network_state.listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            info!("Listening on {:?}", a);
                        }
                        network_state.listening = true;
                    }
                    return Ok(Async::NotReady);
                }
            }
        }
    }));
}

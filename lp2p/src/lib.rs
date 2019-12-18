// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

mod behaviour;
mod config;

use futures::prelude::*;
use libp2p::{core::Multiaddr, Swarm};
use log::info;
use tokio::runtime::TaskExecutor;

#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    listening: bool,
}

pub fn initialize(
    task_executor: TaskExecutor,
    mut network_state: NetworkState,
    peer_ip: Option<String>,
) {
    let (local_key, local_peer_id) = config::configure_key();
    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key);

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut bh = behaviour::Behaviour::new(&local_peer_id);
        bh.floodsub.subscribe(config::hello_topic());
        Swarm::new(transport, bh, local_peer_id.clone())
    };

    let listen_address: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
    // Listen on all interfaces and whatever port the OS assigns
    Swarm::listen_on(&mut swarm, listen_address.clone()).unwrap();
    swarm.kad.add_address(&local_peer_id, listen_address);
    if let Some(peer_ip) = peer_ip {
        Swarm::dial_addr(&mut swarm, peer_ip.parse().unwrap());
    }
    task_executor.spawn(futures::future::poll_fn(move || -> Result<_, ()> {
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

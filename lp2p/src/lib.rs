// Copyright 2019 PolkaX

mod behaviour;
mod config;
mod hello;

use futures::prelude::*;
use libp2p::Swarm;
use tokio::runtime::TaskExecutor;

#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    listenning: bool,
}

pub fn initialize(task_executor: TaskExecutor,
          mut network_state: NetworkState, peer_ip: Option<String>) {
    let (local_key, local_peer_id) = config::configure_key();
    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key);

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut bh = behaviour::Behaviour::new(&local_peer_id);
        config::configure_topic()
            .iter()
            .map(|topic| bh.floodsub.subscribe(topic.clone()));
        Swarm::new(transport, bh, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    if let Some(peer_ip) = peer_ip {
        Swarm::dial_addr(&mut swarm, peer_ip.parse().unwrap());
    }
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

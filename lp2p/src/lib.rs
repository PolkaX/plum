// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

mod behaviour;
mod config;
mod hello;

use std::time::{Duration, Instant};

use futures::prelude::*;
use futures::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use libp2p::{core::Multiaddr, PeerId, Swarm};
use log::{debug, error, info, warn};
use tokio::runtime::TaskExecutor;
use tokio::timer::Interval;

use crate::behaviour::{Behaviour, Event};
use crate::hello::Message as HelloMessage;

#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    listening: bool,
}

pub fn initialize<C: 'static + Send + Sync + chain::Client>(
    task_executor: TaskExecutor,
    mut network_state: NetworkState,
    peer_ip: Option<Multiaddr>,
    client: C,
) {
    let (local_key, local_peer_id) = config::configure_key();
    info!("Local node identity: {}", local_peer_id);

    let (sender, mut receiver): (UnboundedSender<Event>, UnboundedReceiver<Event>) =
        mpsc::unbounded();

    let (kad_sender, mut kad_receiver): (UnboundedSender<bool>, UnboundedReceiver<bool>) =
        mpsc::unbounded();

    // Create a Swarm to manage peers and events
    let mut swarm = {
        // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
        let transport = libp2p::build_development_transport(local_key);

        let mut behaviour = Behaviour::new(&local_peer_id, sender);
        behaviour.floodsub.subscribe(config::hello_topic());

        Swarm::new(transport, behaviour, local_peer_id.clone())
    };

    // TODO: listen on specified address
    let listen_address: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();

    // Listen on all interfaces and whatever port the OS assigns
    match Swarm::listen_on(&mut swarm, listen_address.clone()) {
        Ok(_) => {
            info!("Listening established, address: {}", listen_address);
        }
        Err(err) => {
            panic!(
                "Libp2p was unable to listen on the given listen address {:?}, error: {:?}",
                listen_address, err
            );
        }
    };

    swarm.kad.add_address(&local_peer_id, listen_address);

    let mut dial_addr = |multiaddr: Multiaddr| {
        match Swarm::dial_addr(&mut swarm, multiaddr.clone()) {
            Ok(()) => info!("Dialing libp2p peer, address: {}", multiaddr),
            Err(err) => info!(
                "Could not connect to peer, address: {}, error: {:?}",
                multiaddr, err
            ),
        };
    };

    // TODO: could pass a list of peer_ip,
    // attempt to connect to user-input libp2p nodes
    if let Some(peer_ip) = peer_ip {
        dial_addr(peer_ip);
    }

    let (mut peermgr, peermgr_handle) = peermgr::PeerMgr::new();

    task_executor.spawn(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match receiver.poll().expect("Error polling in receiver channel") {
                Async::Ready(None) | Async::NotReady => break,
                Async::Ready(Some(e)) => {
                    match e {
                        Event::Connecting(peer_id) => {
                            // TODO: mock hello msg
                            let hello_msg =
                                HelloMessage::new(0u8, 1u128, 0u8, local_peer_id.clone().into());

                            let msg = behaviour::GenericMessage::Hello(hello_msg);
                            let data = serde_cbor::to_vec(&msg).expect("Fail to apply serde_cbor");
                            swarm.floodsub.publish(config::hello_topic(), data);
                        }
                        Event::Message(msg) => {
                            let behaviour_msg: behaviour::GenericMessage =
                                serde_cbor::from_slice(&msg.data).expect("Fail to decode cbor");
                            match behaviour_msg {
                                behaviour::GenericMessage::Hello(msg) => {
                                    // TODO handle hello message
                                    let HelloMessage {
                                        heaviest_tip_set,
                                        heaviest_tip_set_weight,
                                        genesis_hash,
                                        sender,
                                    } = msg;

                                    let sender = PeerId::from_bytes(sender.0)
                                        .expect("TODO ensure it won't panic");

                                    if genesis_hash != client.info().genesis_hash {
                                        warn!(
                                            "Our genesis hash: {}, theirs: {}, sender: {:?}",
                                            genesis_hash,
                                            client.info().genesis_hash,
                                            sender,
                                        );
                                        // TODO disconnect
                                        // info!("ban peer_id: {:?}", sender);
                                        // Swarm::ban_peer_id(&mut swarm, sender);
                                        peermgr_handle.remove_peer(sender);
                                        return Ok(Async::NotReady);
                                    }

                                    info!("we are on the same chain! {}", genesis_hash);

                                    // TODO: inform new head

                                    // TODO: add to peermgr
                                    peermgr_handle.add_peer(sender);
                                }
                            }
                            // on FloodsubMessage
                        }
                    }
                }
            }
        }

        loop {
            match kad_receiver
                .poll()
                .expect("Error polling in receiver channel")
            {
                Async::Ready(None) | Async::NotReady => {
                    break;
                }
                Async::Ready(Some(_)) => {
                    info!("[kad_receiver] do swarm.kad.bootstrap...");
                    swarm.kad.bootstrap();
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

    let mut peermgr_interval = Interval::new(Instant::now(), Duration::from_millis(5000));

    task_executor.spawn(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match peermgr.rx.poll() {
                Ok(Async::Ready(Some(action))) => {
                    info!("[peermgr.rx] poll");
                    match action {
                        peermgr::Action::AddPeer(id) => peermgr.on_add_peer(id),
                        peermgr::Action::RemovePeer(id) => peermgr.on_remove_peer(&id),
                    }
                    let _ = kad_sender.unbounded_send(true);
                }
                Ok(Async::Ready(None)) => break,
                Ok(Async::NotReady) => break,
                Err(_err) => break,
            }
        }

        loop {
            match peermgr_interval.poll() {
                Ok(Async::Ready(Some(_instant))) => {
                    info!("[peermgr_interval] poll");
                    let peer_cnt = peermgr.get_peer_count();
                    if peer_cnt < peermgr.min_fil_peers as usize {
                        info!(
                            "current peer count: {:?}, peermgr.min_fil_peers: {:?}, send bootstrap request",
                            peer_cnt, peermgr.min_fil_peers
                        );
                        // TODO: use a timer delay
                        // if timer_id != -1
                        //  stop(timer_id)
                        // endif
                        // let timer_id = timer_start(30, { -> kad_sender.unbounded_send(true)});
                        let _ = kad_sender.unbounded_send(true);
                    } else if peer_cnt > peermgr.max_fil_peers as usize {
                        info!(
                            "peermgr.max_fil_peers {} reached, current peer count: {}",
                            peermgr.max_fil_peers, peer_cnt
                        );
                    } else {
                        info!(
                            "current peer count: {:?}, peermgr.min_fil_peers: {:?}, peermgr.max_fil_peers: {:?}",
                            peer_cnt, peermgr.min_fil_peers, peermgr.max_fil_peers
                        );
                    }
                }
                Ok(Async::Ready(None)) => break,
                Ok(Async::NotReady) => break,
                Err(_err) => break,
            }
        }

        Ok(Async::NotReady)
    }));
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use futures::stream::Stream;
use futures::{Async, Future};
use libp2p::gossipsub::Topic;
use log::{debug, info, warn};
use plum_libp2p::config::Libp2pConfig;
use plum_libp2p::rpc::RPCEvent;
use plum_libp2p::service::{Libp2pEvent, Libp2pService};
use plum_libp2p::PeerId;
use std::sync::{Arc, Mutex};
use tokio::runtime::TaskExecutor;
use tokio::sync::mpsc;

use crate::message_handler::{HandlerMessage, MessageHandler};

pub enum NetworkMessage {
    PubsubMessage { topics: Topic, message: Vec<u8> },
    RPC(PeerId, RPCEvent),
}

pub struct Service {
    pub libp2p: Arc<Mutex<Libp2pService>>,
}

impl Service {
    pub fn spawn(
        config: &Libp2pConfig,
        executor: &TaskExecutor,
    ) -> (
        Self,
        mpsc::UnboundedSender<NetworkMessage>,
        tokio::sync::oneshot::Sender<u8>,
    ) {
        let (network_send, network_recv) = mpsc::unbounded_channel::<NetworkMessage>();

        let message_handler_send = MessageHandler::spawn(network_send.clone(), executor)
            .expect("Failed to spawn message handler thread");

        let libp2p_service = Arc::new(Mutex::new(Libp2pService::new(config)));

        let exit_tx = spawn_service(
            libp2p_service.clone(),
            network_recv,
            message_handler_send,
            executor,
        );

        (
            Self {
                libp2p: libp2p_service,
            },
            network_send,
            exit_tx,
        )
    }
}

fn spawn_service(
    libp2p_service: Arc<Mutex<Libp2pService>>,
    network_recv: mpsc::UnboundedReceiver<NetworkMessage>,
    message_handler_send: mpsc::UnboundedSender<HandlerMessage>,
    executor: &TaskExecutor,
) -> tokio::sync::oneshot::Sender<u8> {
    let (network_exit, exit_rx) = tokio::sync::oneshot::channel();

    executor.spawn(
        network_service(libp2p_service, network_recv, message_handler_send)
            .select(exit_rx.then(|_| Ok(())))
            .then(move |_| Ok(())),
    );

    network_exit
}

enum Error {}

fn network_service(
    libp2p_service: Arc<Mutex<Libp2pService>>,
    mut network_recv: mpsc::UnboundedReceiver<NetworkMessage>,
    mut message_handler_send: mpsc::UnboundedSender<HandlerMessage>,
) -> impl futures::Future<Item = (), Error = Error> {
    futures::future::poll_fn(move || -> Result<_, _> {
        loop {
            match network_recv.poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    NetworkMessage::RPC(peer_id, rpc_event) => {
                        debug!(
                            "Sending RPC, peer_id: {:?}, rpc_event: {:?}",
                            peer_id, rpc_event
                        );
                        libp2p_service
                            .lock()
                            .unwrap()
                            .swarm
                            .send_rpc(peer_id, rpc_event);
                    }
                    NetworkMessage::PubsubMessage { topics, message } => {
                        debug!(
                            "Publishing NetworkMessage, topics: {:?}, message: {:?}",
                            topics, message
                        );
                        libp2p_service
                            .lock()
                            .unwrap()
                            .swarm
                            .publish(&topics, message);
                    }
                },
                Ok(Async::NotReady) => break,
                _ => break,
            }
        }
        loop {
            match libp2p_service.lock().unwrap().poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    Libp2pEvent::PubsubMessage {
                        id,
                        source,
                        topics,
                        data,
                    } => {
                        info!(
                            "libp2p_service received PubsubMessage, source: {:?}, topics: {:?}, message: {:?}",
                            source, topics, data
                        );

                        if message_handler_send
                            .try_send(HandlerMessage::PubsubMessage {
                                id,
                                source,
                                topics,
                                data,
                            })
                            .is_err()
                        {
                            warn!("Failed to send PubsubMessage");
                        }
                    }
                    Libp2pEvent::RPC(peer, rpc_event) => {
                        if message_handler_send
                            .try_send(HandlerMessage::RPC(peer.clone(), rpc_event))
                            .is_err()
                        {
                            warn!("Failed to send RPC HandlerMessage from {}", peer);
                        }
                    }
                    Libp2pEvent::HelloSubscribed(peer) => {
                        if message_handler_send
                            .try_send(HandlerMessage::SayHello(peer.clone()))
                            .is_err()
                        {
                            warn!("Failed to send SayHello HandlerMessage from {}", peer);
                        }
                    }
                },
                Ok(Async::Ready(None)) => unreachable!("Stream never ends"),
                Ok(Async::NotReady) => break,
                _ => break,
            }
        }
        Ok(Async::NotReady)
    })
}

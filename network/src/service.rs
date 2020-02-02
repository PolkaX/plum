use futures::stream::Stream;
use futures::{Async, Future};
use libp2p::gossipsub::Topic;
use plum_libp2p::config::Libp2pConfig;
use plum_libp2p::service::{Libp2pEvent, Libp2pService};
use slog::{warn, Logger};
use std::sync::{Arc, Mutex};
use tokio::runtime::TaskExecutor;
use tokio::sync::mpsc;

use crate::message_handler::{HandlerMessage, MessageHandler};

/// Ingress events to the NetworkService
pub enum NetworkMessage {
    PubsubMessage { topics: Topic, message: Vec<u8> },
}

/// Receives commands through a channel which communicates with Libp2p.
/// It also listens to the Libp2p service for messages.
pub struct NetworkService {
    /// Libp2p instance
    pub libp2p: Arc<Mutex<Libp2pService>>,
}

impl NetworkService {
    /// Starts a Libp2pService with a given config, UnboundedSender, and tokio executor.
    /// Returns an UnboundedSender channel so messages can come in.
    pub fn new(
        config: &Libp2pConfig,
        executor: &TaskExecutor,
    ) -> (
        Self,
        mpsc::UnboundedSender<NetworkMessage>,
        tokio::sync::oneshot::Sender<u8>,
    ) {
        let (network_send, network_recv) = mpsc::unbounded_channel::<NetworkMessage>();

        let message_handler_send = MessageHandler::spawn(network_send.clone(), executor)
            .expect("Fail to spawn message handler thread");

        let libp2p_service = Arc::new(Mutex::new(Libp2pService::new(config)));

        let exit_tx = spawn_service(
            libp2p_service.clone(),
            network_recv,
            message_handler_send,
            executor,
        );

        (
            NetworkService {
                libp2p: libp2p_service,
            },
            network_send,
            exit_tx,
        )
    }
}

enum Error {}

/// Spawns the NetworkService service.
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

fn network_service(
    libp2p_service: Arc<Mutex<Libp2pService>>,
    mut network_recv: mpsc::UnboundedReceiver<NetworkMessage>,
    mut message_handler_send: mpsc::UnboundedSender<HandlerMessage>,
) -> impl futures::Future<Item = (), Error = Error> {
    futures::future::poll_fn(move || -> Result<_, _> {
        loop {
            match network_recv.poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    NetworkMessage::PubsubMessage { topics, message } => {
                        log::info!(
                            "----------- network_recv received topics: {:?}, message: {:?}",
                            topics,
                            message
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
                        source,
                        topics,
                        message,
                    } => {
                        log::info!(
                            "----------- libp2p_service received source: {:?}, topics: {:?}, message: {:?}",
                            source,
                            topics,
                            message
                        );
                        /*
                        if message_handler_send
                            .try_send(HandlerMessage::PubsubMessage {
                                source,
                                topics,
                                message,
                            })
                            .is_err()
                        {
                            log::warn!("Cant handle message");
                        }
                        */
                    }
                    Libp2pEvent::Hello(peer) => {
                        log::info!("------------ hello -----------");
                        if message_handler_send
                            .try_send(HandlerMessage::Hello(peer))
                            .is_err()
                        {
                            log::warn!("Cant handle hello message");
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

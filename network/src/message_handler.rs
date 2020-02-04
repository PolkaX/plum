use anyhow::Result;
use futures::future::Future;
use futures::stream::Stream;
use log::{debug, error};
use plum_libp2p::rpc::methods::BlocksByRangeRequest;
use plum_libp2p::rpc::{RPCEvent, RPCRequest};
use plum_libp2p::{config::HELLO_TOPIC, MessageId, PeerId, TopicHash};
use tokio::sync::mpsc;

use crate::service::NetworkMessage;

pub struct MessageHandler {
    network_send: mpsc::UnboundedSender<NetworkMessage>,
}

/// Types of messages the handler can receive.
#[derive(Debug)]
pub enum HandlerMessage {
    SayHello(PeerId),
    RPC(PeerId, RPCEvent),
    PubsubMessage {
        id: MessageId,
        source: PeerId,
        topics: Vec<TopicHash>,
        data: Vec<u8>,
    },
}

impl MessageHandler {
    pub fn spawn(
        network_send: mpsc::UnboundedSender<NetworkMessage>,
        executor: &tokio::runtime::TaskExecutor,
    ) -> Result<mpsc::UnboundedSender<HandlerMessage>> {
        let (handler_send, handler_recv) = mpsc::unbounded_channel();

        // generate the Message handler
        let mut handler = MessageHandler { network_send };

        // spawn handler task and move the message handler instance into the spawned thread
        executor.spawn(
            handler_recv
                .for_each(move |msg| Ok(handler.handle_message(msg)))
                .map_err(move |_| {
                    debug!("Network message handler terminated.");
                }),
        );

        Ok(handler_send)
    }

    fn handle_message(&mut self, message: HandlerMessage) {
        match message {
            HandlerMessage::SayHello(_peer_id) => {
                // TODO:
                // 1. get current status of local node
                // 2. publish the hello message
                if self
                    .network_send
                    .try_send(NetworkMessage::HelloMessage(
                        b"hello message sent on say hello".to_vec(),
                    ))
                    .is_err()
                {
                    error!("Failed to send HelloMessage");
                }
            }
            HandlerMessage::RPC(peer, rpc_event) => match rpc_event {
                RPCEvent::Request(id, request) => {
                    println!(
                        "---------------- handle rpc event Request: id: {}, request: {:?}",
                        id, request
                    );
                    match request {
                        RPCRequest::Status(s) => {
                            println!("---- handle status ");
                        }
                        RPCRequest::Goodbye(s) => {
                            println!("---- handle goodbye ");
                        }
                        RPCRequest::BlocksByRoot(s) => {
                            println!("---- handle blocks by root ");
                        }
                        RPCRequest::BlocksByRange(req) => {
                            println!("------ handle BlocksByRange: {:?}", req);
                        }
                    }
                }
                _ => {
                    println!("---------------- handle rpc event other Request");
                }
            },
            HandlerMessage::PubsubMessage {
                id,
                source,
                topics,
                data,
            } => {
                debug!(
                    "handle PubsubMessage, id: {:?}, source: {:?}, topics: {:?}, data: {:?}",
                    id, source, topics, data
                );
                // TODO: Dispatch hello/blocks/messages message
                for topic in topics {
                    if topic == TopicHash::from_raw(HELLO_TOPIC) {
                        // TODO: handle hello message, send the sync request if all checks passed.
                        // send the sync service.
                        //
                        //
                        self.network_send.try_send(NetworkMessage::RPC(
                            source.clone(),
                            RPCEvent::Request(
                                0usize,
                                RPCRequest::BlocksByRange(BlocksByRangeRequest {
                                    start_slot: 666u64,
                                    count: 777u64,
                                    step: 111u64,
                                }),
                            ),
                        ));
                    }
                }
            }
        }
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::Result;
use futures::future::Future;
use futures::stream::Stream;
use log::{debug, error};
use plum_libp2p::rpc::methods::BlocksByRangeRequest;
use plum_libp2p::rpc::{RPCEvent, RPCRequest, RequestId};
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

    fn on_request(&self, peer: PeerId, id: RequestId, request: RPCRequest) {
        match request {
            RPCRequest::Status(status) => {
                debug!(
                    "handling RPC Status message, peer:{:?}, id: {}, request: {:?}",
                    peer, id, status
                );
            }
            RPCRequest::Goodbye(goodbye) => {
                debug!(
                    "handling RPC Goodbye message, peer:{:?}, id: {}, request: {:?}",
                    peer, id, goodbye
                );
            }
            RPCRequest::BlocksByRoot(blocks_by_root) => {
                debug!(
                    "handling RPC BlocksByRoot message, peer:{:?}, id: {}, request: {:?}",
                    peer, id, blocks_by_root
                );
            }
            RPCRequest::BlocksByRange(blocks_by_range) => {
                debug!(
                    "handling RPC BlocksByRange message, peer:{:?}, id: {}, request: {:?}",
                    peer, id, blocks_by_range
                );
            }
        }
    }

    fn on_hello_message(&mut self, peer: PeerId) {
        // TODO:
        // 1. get current status of local node
        // 2. publish the hello message

        if self
            .network_send
            .try_send(NetworkMessage::HelloMessage(
                b"dummy hello message sent on say hello".to_vec(),
            ))
            .is_err()
        {
            error!("Failed to send HelloMessage to {:?}", peer);
        }
    }

    fn on_pubsub_message(
        &mut self,
        id: MessageId,
        source: PeerId,
        topics: Vec<TopicHash>,
        data: Vec<u8>,
    ) {
        debug!(
            "handling PubsubMessage, id: {:?}, source: {:?}, topics: {:?}, data: {:?}",
            id, source, topics, data
        );
        // TODO: Dispatch hello/blocks/messages message
        for topic in topics {
            if topic == TopicHash::from_raw(HELLO_TOPIC) {
                // TODO: handle hello message, send the sync request if all checks passed.
                //
                // send the sync service.
                if self
                    .network_send
                    .try_send(NetworkMessage::RPC(
                        source.clone(),
                        RPCEvent::Request(
                            0usize,
                            RPCRequest::BlocksByRange(BlocksByRangeRequest {
                                start_slot: 666u64,
                                count: 777u64,
                                step: 111u64,
                            }),
                        ),
                    ))
                    .is_err()
                {
                    error!("Failed to send RPC Block Request message");
                }
            }
        }
    }

    fn on_rpc_message(&mut self, peer: PeerId, event: RPCEvent) {
        match event {
            RPCEvent::Request(id, request) => self.on_request(peer, id, request),
            RPCEvent::Response(id, err_response) => {
                debug!(
                    "handling RPC Response message, peer:{:?}, id: {}, err_response: {:?}",
                    peer, id, err_response
                );
            }
            RPCEvent::Error(id, err) => {
                debug!(
                    "handling RPC Error message, peer:{:?}, id: {}, err: {:?}",
                    peer, id, err
                );
            }
        }
    }

    fn handle_message(&mut self, message: HandlerMessage) {
        match message {
            HandlerMessage::SayHello(peer) => self.on_hello_message(peer),
            HandlerMessage::RPC(peer, rpc_event) => self.on_rpc_message(peer, rpc_event),
            HandlerMessage::PubsubMessage {
                id,
                source,
                topics,
                data,
            } => self.on_pubsub_message(id, source, topics, data),
        }
    }
}

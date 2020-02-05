// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::Result;
use futures::future::Future;
use futures::stream::Stream;
use log::{debug, error};
use plum_libp2p::rpc::methods::BlockSyncRequest;
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

    fn process_request(&self, peer: PeerId, id: RequestId, request: RPCRequest) {
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
            RPCRequest::BlockSyncRequest(block_sync_request) => {
                debug!(
                    "handling RPC BlockSyncRequest message, peer:{:?}, id: {}, request: {:?}",
                    peer, id, block_sync_request
                );
            }
        }
    }

    fn on_say_hello(&mut self, peer: PeerId) {
        // TODO: https://github.com/filecoin-project/lotus/blob/e7a1be4dde/node/hello/hello.go#L114
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

    fn process_hello_message(&mut self, _id: MessageId, source: PeerId, _data: Vec<u8>) {
        // TODO:
        // https://github.com/filecoin-project/lotus/blob/e7a1be4dde/node/hello/hello.go#L62

        let dummy_request = RPCRequest::BlockSyncRequest(BlockSyncRequest {
            start: Vec::new(),
            length: 777u64,
            options: 111u64,
        });

        if self
            .network_send
            .try_send(NetworkMessage::RPC(
                source,
                RPCEvent::Request(0usize, dummy_request),
            ))
            .is_err()
        {
            error!("Failed to send RPC Block Request message");
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
        // Dispatch hello/blocks/messages message
        for topic in topics {
            if topic == TopicHash::from_raw(HELLO_TOPIC) {
                self.process_hello_message(id.clone(), source.clone(), data.clone());
            }
        }
    }

    fn on_rpc(&mut self, peer: PeerId, event: RPCEvent) {
        match event {
            RPCEvent::Request(id, request) => self.process_request(peer, id, request),
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
            HandlerMessage::SayHello(peer) => self.on_say_hello(peer),
            HandlerMessage::RPC(peer, rpc_event) => self.on_rpc(peer, rpc_event),
            HandlerMessage::PubsubMessage {
                id,
                source,
                topics,
                data,
            } => self.on_pubsub_message(id, source, topics, data),
        }
    }
}

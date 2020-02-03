use anyhow::Result;
use futures::future::Future;
use futures::stream::Stream;
use log::debug;
use plum_libp2p::{MessageId, PeerId, TopicHash};
use tokio::sync::mpsc;

use crate::service::NetworkMessage;

pub struct MessageHandler {
    network_send: mpsc::UnboundedSender<NetworkMessage>,
}

/// Types of messages the handler can receive.
#[derive(Debug)]
pub enum HandlerMessage {
    SayHello(PeerId),
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
            HandlerMessage::SayHello(peer_id) => {
                println!("============== handle_message hello");
                // TODO: get current status of local node
                self.network_send.try_send(NetworkMessage::HelloMessage(
                    b"hello message sent on say hello".to_vec(),
                ));
            }
            HandlerMessage::PubsubMessage {
                id,
                source,
                topics,
                data,
            } => {
                println!("============== handle pubsub message id: {:?}, source: {:?}, topics: {:?}, data: {:?}", id, source, topics, data);
            }
        }
    }
}

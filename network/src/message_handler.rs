use anyhow::Result;
use futures::future::Future;
use futures::stream::Stream;
use plum_libp2p::PeerId;
use tokio::sync::mpsc;

use crate::service::NetworkMessage;

pub struct MessageHandler;

/// Types of messages the handler can receive.
#[derive(Debug)]
pub enum HandlerMessage {
    SayHello(PeerId),
    // PubsubMessage(MessageId, PeerId, PubsubMessage),
}

impl MessageHandler {
    /// Initializes and runs the MessageHandler.
    pub fn spawn(
        network_send: mpsc::UnboundedSender<NetworkMessage>,
        executor: &tokio::runtime::TaskExecutor,
    ) -> Result<mpsc::UnboundedSender<HandlerMessage>> {
        let (handler_send, handler_recv) = mpsc::unbounded_channel();

        // generate the Message handler
        let mut handler = MessageHandler;

        // spawn handler task and move the message handler instance into the spawned thread
        executor.spawn(
            handler_recv
                .for_each(move |msg| Ok(handler.handle_message(msg)))
                .map_err(move |_| {
                    log::debug!("Network message handler terminated.");
                }),
        );

        Ok(handler_send)
    }

    /// Handle all messages incoming from the network service.
    fn handle_message(&mut self, message: HandlerMessage) {
        match message {
            // A peer has disconnected
            HandlerMessage::SayHello(peer_id) => {
                println!("============== handle_message hello");
            }
        }
    }
}

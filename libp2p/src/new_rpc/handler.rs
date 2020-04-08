// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use std::collections::HashMap;

use futures_codec::Framed;
use libp2p::{
    core::{InboundUpgrade, OutboundUpgrade},
    swarm::{
        protocols_handler::{
            InboundUpgradeSend, KeepAlive, OneShotHandler, OutboundUpgradeSend, ProtocolsHandler,
            ProtocolsHandlerEvent, ProtocolsHandlerUpgrErr, SubstreamProtocol,
        },
        NegotiatedSubstream,
    },
};
use smallvec::SmallVec;

use crate::new_rpc::behaviour::RpcEvent;
use crate::new_rpc::protocol::{
    InboundCodec, InboundProtocol, OutboundCodec, OutboundProtocol, RequestId, RpcError,
    RpcMessage, RpcRequest, RpcResponse, RpcResponseFailure, RpcResponseSuccess,
};

/// Implementation of `ProtocolsHandler` that opens a new substream for each individual message.
///
/// Reference from `OneShotHandler` and `GossipsubHandler`.
pub struct RpcHandler {
    /// The upgrade for inbound substreams.
    listen_protocol: SubstreamProtocol<InboundProtocol>,
    /// If `Some`, something bad happened and we should shut down the handler with an error.
    pending_error: Option<(RequestId, ProtocolsHandlerUpgrErr<RpcError>)>,
    /// Queue of events to produce in `poll()`.
    events_out: SmallVec<[RpcMessage; 4]>,
    /// Queue of outbound substreams to open.
    dial_queue: SmallVec<[RpcMessage; 4]>,
    /// Current number of concurrent outbound substreams being opened.
    dial_negotiated: u32,
    /// Maximum number of concurrent outbound substreams being opened. Value is never modified.
    max_dial_negotiated: u32,
    /// Value to return from `connection_keep_alive`.
    keep_alive: KeepAlive,
    /// The configuration container for the handler
    config: RpcHandlerConfig,

    /// Map of current inbound substreams awaiting processing a response to the request.
    /// The `RequestId` is maintained by the application sending the request.
    inbound_substreams: HashMap<RequestId, InboundSubstreamState>,
    /// Map of outbound substreams that need to be driven to completion.
    /// The `RequestId` is maintained by the application sending the request.
    outbound_substreams: HashMap<RequestId, OutboundSubstreamState>,
    /// Sequential ID for new substreams.
    current_substream_id: RequestId,
}

type InboundFramed = Framed<NegotiatedSubstream, InboundCodec>;
type OutboundFramed = Framed<NegotiatedSubstream, OutboundCodec>;

/// State of the inbound substream
enum InboundSubstreamState {
    /// Request has been received, local peer is waiting to send a response to the remote peer.
    PendingSendResponse {
        /// The framed negotiated substream used to send the response.
        substream: InboundFramed,
        /// The time when the substream is closed.
        timeout: Instant,
    },
    /// An error occurred during processing.
    Poisoned,
}

/// State of the outbound substream
enum OutboundSubstreamState {
    /// Local is waiting to send a response to the remote.
    PendingSendResponse {
        substream: InboundFramed,
        response: RpcResponse,
    },
    /// Request has been sent, local is waiting to received a response from the remote.
    PendingRecvResponse {
        substream: OutboundFramed,
        event: RpcMessage, // RPC Response or RPC Error
        timeout: Instant,
    },
    /// An error occurred during processing.
    Poisoned,
}

impl RpcHandler {
    /// Creates a `RpcHandler`.
    #[inline]
    pub fn new(config: RpcHandlerConfig) -> Self {
        RpcHandler {
            listen_protocol: SubstreamProtocol::new(InboundProtocol),
            pending_error: None,
            events_out: SmallVec::new(),
            dial_queue: SmallVec::new(),
            dial_negotiated: 0,
            max_dial_negotiated: 8,
            keep_alive: KeepAlive::Yes,
            config,

            inbound_substreams: HashMap::new(),
            outbound_substreams: HashMap::new(),
            current_substream_id: 1,
        }
    }

    /// Returns the number of pending requests.
    #[inline]
    pub fn pending_requests(&self) -> u32 {
        self.dial_negotiated + self.dial_queue.len() as u32
    }

    /// Opens an outbound substream with `message`.
    #[inline]
    pub fn send_request(&mut self, message: RpcMessage) {
        self.keep_alive = KeepAlive::Yes;
        self.dial_queue.push(message);
    }
}

impl Default for RpcHandler {
    #[inline]
    fn default() -> Self {
        RpcHandler::new(RpcHandlerConfig::default())
    }
}

/// Configuration parameters for the `RpcHandler`
#[derive(Debug)]
pub struct RpcHandlerConfig {
    /// After the given duration has elapsed, an inactive connection will shutdown.
    pub inactive_timeout: Duration,
    /// Timeout duration for each newly opened outbound substream.
    pub substream_timeout: Duration,
}

impl Default for RpcHandlerConfig {
    fn default() -> Self {
        let inactive_timeout = Duration::from_secs(10);
        let substream_timeout = Duration::from_secs(10);
        RpcHandlerConfig {
            inactive_timeout,
            substream_timeout,
        }
    }
}

impl ProtocolsHandler for RpcHandler {
    /// Custom event that can be received from the outside.
    type InEvent = RpcMessage;
    /// Custom event that can be produced by the handler and that will be returned to the outside.
    type OutEvent = RpcMessage;
    /// The type of errors returned by [`ProtocolsHandler::poll`].
    type Error = RpcError;
    /// The inbound upgrade for the protocol(s) used by the handler.
    type InboundProtocol = InboundProtocol;
    /// The outbound upgrade for the protocol(s) used by the handler.
    type OutboundProtocol = OutboundProtocol;
    /// The type of additional information passed to an `OutboundSubstreamRequest`.
    type OutboundOpenInfo = RpcMessage;

    #[inline]
    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol> {
        self.listen_protocol.clone()
    }

    /// Injects the output of a successful upgrade on a new inbound substream.
    #[inline]
    fn inject_fully_negotiated_inbound(
        &mut self,
        out: <Self::InboundProtocol as InboundUpgrade<NegotiatedSubstream>>::Output,
    ) {
        let (request, substream) = out;

        // New inbound request. Store the stream.
        let inbound_substream_state = InboundSubstreamState::PendingSendResponse {
            substream,
            timeout: Instant::now() + self.config.inactive_timeout,
        };
        self.inbound_substreams
            .insert(self.current_substream_id, inbound_substream_state);

        /*
        // If we're shutting down the connection for inactivity, reset the timeout.
        if !self.keep_alive.is_yes() {
            self.keep_alive = KeepAlive::Until(Instant::now() + self.config.inactive_timeout);
        }
        */

        self.events_out
            .push(RpcMessage::Request(self.current_substream_id, request));
        self.current_substream_id += 1;
    }

    /// Injects the output of a successful upgrade on a new outbound substream.
    ///
    /// The second argument is the information that was previously passed to
    /// [`ProtocolsHandlerEvent::OutboundSubstreamRequest`].
    #[inline]
    fn inject_fully_negotiated_outbound(
        &mut self,
        out: <Self::OutboundProtocol as OutboundUpgrade<NegotiatedSubstream>>::Output,
        info: Self::OutboundOpenInfo,
    ) {
        self.dial_negotiated -= 1;

        if self.dial_negotiated == 0 && self.dial_queue.is_empty() {
            self.keep_alive = KeepAlive::Until(Instant::now() + self.config.inactive_timeout);
        }

        self.events_out.push(out.into());
    }

    /// Injects an event coming from the outside in the handler.
    #[inline]
    fn inject_event(&mut self, event: Self::InEvent) {
        match event {
            RpcMessage::Request(_, _) => self.send_request(event),
            // Note: If the substream has closed due to inactivity or the substream is in the
            // wrong state, a response will fail silently.
            RpcMessage::Response(request_id, response) => {
                // check if the stream matching the response still exists
                if let Some(inbound_substream_state) = self.inbound_substreams.remove(&request_id) {
                    match inbound_substream_state {
                        InboundSubstreamState::PendingSendResponse { substream, timeout } => {}
                    }
                    // only send one response per stream. This must be in the waiting state
                    self.outbound_substreams
                        .push(OutboundSubstreamState::PendingSendResponse {
                            substream: inbound_substream_state.substream,
                            response: inbound_substream_state,
                        });
                }
            }
            // We do not send errors as responses
            RpcMessage::Error(_, _) => {}
        }
    }

    /// Indicates to the handler that upgrading a substream to the given protocol has failed.
    #[inline]
    fn inject_dial_upgrade_error(
        &mut self,
        info: Self::OutboundOpenInfo,
        error: ProtocolsHandlerUpgrErr<
            <Self::OutboundProtocol as OutboundUpgrade<NegotiatedSubstream>>::Error,
        >,
    ) {
        let request_id = match info {
            RpcMessage::Request(request_id, _) => request_id,
            RpcMessage::Response(_, _) => 0,
            RpcMessage::Error(_, _) => 0,
        };
        if self.pending_error.is_none() {
            self.pending_error = Some((request_id, error));
        }
    }

    /// Returns until when the connection should be kept alive.
    #[inline]
    fn connection_keep_alive(&self) -> KeepAlive {
        self.keep_alive
    }

    /// Should behave like `Stream::poll()`.
    fn poll(
        &mut self,
        _cx: &mut Context,
    ) -> Poll<
        ProtocolsHandlerEvent<
            Self::OutboundProtocol,
            Self::OutboundOpenInfo,
            Self::OutEvent,
            Self::Error,
        >,
    > {
        if let Some(err) = self.pending_error.take() {
            return Poll::Ready(ProtocolsHandlerEvent::Close(err));
        }

        if !self.events_out.is_empty() {
            return Poll::Ready(ProtocolsHandlerEvent::Custom(self.events_out.remove(0)));
        } else {
            self.events_out.shrink_to_fit();
        }

        if !self.dial_queue.is_empty() {
            if self.dial_negotiated < self.max_dial_negotiated {
                self.dial_negotiated += 1;
                return Poll::Ready(ProtocolsHandlerEvent::OutboundSubstreamRequest {
                    protocol: SubstreamProtocol::new(self.dial_queue.remove(0))
                        .with_timeout(self.config.substream_timeout),
                    info: (),
                });
            }
        } else {
            self.dial_queue.shrink_to_fit();
        }

        Poll::Pending
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use fnv::FnvHashMap;
use futures_codec::Framed;
use libp2p::{
    core::{InboundUpgrade, OutboundUpgrade},
    swarm::{
        protocols_handler::{
            InboundUpgradeSend, KeepAlive, OutboundUpgradeSend, ProtocolsHandler,
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
    inbound_substreams: FnvHashMap<RequestId, InboundSubstreamState>,
    /// Map of outbound substreams that need to be driven to completion.
    /// The `RequestId` is maintained by the application sending the request.
    outbound_substreams: FnvHashMap<RequestId, OutboundSubstreamState>,
    /// Sequential ID for new substreams.
    current_substream_id: RequestId,
}

type InboundFramed = Framed<NegotiatedSubstream, InboundCodec>;
type OutboundFramed = Framed<NegotiatedSubstream, OutboundCodec>;

/// State of the inbound substream
/// Either waiting for a response, or in the process of sending.
enum InboundSubstreamState {
    /// A request has been received, pending sending back the response.
    ResponsePendingSend {
        /// The framed negotiated substream used to send the response.
        substream: InboundFramed,
        /// The time when the substream is closed.
        timeout: Instant,
    },
    /// The substream is attempting to shutdown.
    Closing(InboundFramed),
    /// An error occurred during processing.
    Poisoned,
}

/// State of the outbound substream
enum OutboundSubstreamState {
    /// A request has been sent, and we are awaiting a response.
    RequestPendingResponse {
        /// The framed negotiated substream used to receive the response.
        substream: OutboundFramed,
        /// Keeps track of the actual request sent.
        request: RpcRequest,
    },
    /// Closing an outbound substream.
    Closing(OutboundFramed),
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

            inbound_substreams: FnvHashMap::default(),
            outbound_substreams: FnvHashMap::default(),
            current_substream_id: 1,
        }
    }

    /// Returns the number of pending requests.
    #[inline]
    pub fn pending_requests(&self) -> u32 {
        self.dial_negotiated + self.dial_queue.len() as u32
    }

    /// Returns a reference to the listen protocol configuration.
    ///
    /// > **Note**: If you modify the protocol, modifications will only applies to future inbound
    /// >           substreams, not the ones already being negotiated.
    #[inline]
    pub fn listen_protocol_ref(&self) -> &SubstreamProtocol<InboundProtocol> {
        &self.listen_protocol
    }

    /// Returns a mutable reference to the listen protocol configuration.
    ///
    /// > **Note**: If you modify the protocol, modifications will only applies to future inbound
    /// >           substreams, not the ones already being negotiated.
    #[inline]
    pub fn listen_protocol_mut(&mut self) -> &mut SubstreamProtocol<InboundProtocol> {
        &mut self.listen_protocol
    }

    /// Opens an outbound substream with a `message`.
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
        // If we're shutting down the connection for inactivity, reset the timeout.
        // update the keep alive timeout if there are no more remaining outbound streams.
        if !self.keep_alive.is_yes() {
            self.keep_alive = KeepAlive::Until(Instant::now() + self.config.inactive_timeout);
        }

        let (request, substream) = out;
        // New inbound request. Store the substream used to send back the response.
        let inbound_substream_state = InboundSubstreamState::ResponsePendingSend {
            substream,
            timeout: Instant::now() + self.config.inactive_timeout,
        };
        self.inbound_substreams
            .insert(self.current_substream_id, inbound_substream_state);

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

        if self.dial_negotiated == 0
            && self.dial_queue.is_empty()
            && self.outbound_substreams.is_empty()
        {
            self.keep_alive = KeepAlive::Until(Instant::now() + self.config.inactive_timeout);
        } else {
            self.keep_alive = KeepAlive::Yes;
        }

        match info {
            RpcMessage::Request(request_id, request) => {
                if request.expect_response() {
                    // New outbound request. Store the substream used to receive the response.
                    let outbound_substream_state = OutboundSubstreamState::RequestPendingResponse {
                        substream: out,
                        request,
                    };
                    self.outbound_substreams
                        .insert(request_id, outbound_substream_state);
                }
            }
            RpcMessage::Response(_, _) => {} // response is not expected, drop the stream
        }
    }

    /// Injects an event coming from the outside in the handler.
    #[inline]
    fn inject_event(&mut self, event: Self::InEvent) {
        match event {
            RpcMessage::Request(_, _) => self.send_request(event),
            RpcMessage::Response(_request_id, _response) => {}
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
        _cx: &mut Context<'_>,
    ) -> Poll<
        ProtocolsHandlerEvent<
            Self::OutboundProtocol,
            Self::OutboundOpenInfo,
            Self::OutEvent,
            Self::Error,
        >,
    > {
        unimplemented!()
    }
}

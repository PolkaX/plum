// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use futures::{
    future::{self, FutureResult},
    sink, stream, Sink, Stream,
};
use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, ProtocolName, UpgradeInfo};
use serde::{Deserialize, Serialize};
use std::io;
use std::time::Duration;
use tokio::codec::Framed;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::prelude::*;
use tokio::timer::timeout;
use tokio::util::FutureExt;
use tokio_io_timeout::TimeoutStream;

use super::methods::*;
use crate::rpc::{
    codec::{InboundCodec, OutboundCodec},
    methods::ResponseTermination,
};

// TODO: limited size
/// The maximum bytes that can be sent across the RPC.
const MAX_RPC_SIZE: usize = 4_194_304; // 4M
/// The protocol prefix the RPC protocol id.
const PROTOCOL_PREFIX: &str = "/fil/plum/req";
/// Time allowed for the first byte of a request to arrive before we time out (Time To First Byte).
const TTFB_TIMEOUT: u64 = 5;
/// The number of seconds to wait for the first bytes of a request once a protocol has been
/// established before the stream is terminated.
const REQUEST_TIMEOUT: u64 = 15;

/// Protocol names to be used.
/// The Status protocol name.
pub const RPC_STATUS: &str = "status";
/// The Goodbye protocol name.
pub const RPC_GOODBYE: &str = "goodbye";
/// The `BlockSyncRequest` protocol name.
pub const RPC_BLOCK_SYNC_REQUEST: &str = "plum_block_sync_request";

const CBOR: &str = "cbor";

#[derive(Debug, Clone)]
pub struct RPCProtocol;

impl UpgradeInfo for RPCProtocol {
    type Info = ProtocolId;
    type InfoIter = Vec<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        vec![
            ProtocolId::new(RPC_STATUS, "1", CBOR),
            ProtocolId::new(RPC_GOODBYE, "1", CBOR),
            ProtocolId::new(RPC_BLOCK_SYNC_REQUEST, "1", CBOR),
        ]
    }
}

/// Tracks the types in a protocol id.
#[derive(Clone)]
pub struct ProtocolId {
    /// The rpc message type/name.
    pub message_name: String,

    /// The version of the RPC.
    pub version: String,

    /// The encoding of the RPC.
    pub encoding: String,

    /// The protocol id that is formed from the above fields.
    protocol_id: String,
}

/// An RPC protocol ID.
impl ProtocolId {
    pub fn new(message_name: &str, version: &str, encoding: &str) -> Self {
        let protocol_id = format!(
            "{}/{}/{}/{}",
            PROTOCOL_PREFIX, message_name, version, encoding
        );

        ProtocolId {
            message_name: message_name.into(),
            version: version.into(),
            encoding: encoding.into(),
            protocol_id,
        }
    }
}

impl ProtocolName for ProtocolId {
    fn protocol_name(&self) -> &[u8] {
        self.protocol_id.as_bytes()
    }
}

/* Inbound upgrade */

// The inbound protocol reads the request, decodes it and returns the stream to the protocol
// handler to respond to once ready.

pub type InboundOutput<TSocket> = (RPCRequest, InboundFramed<TSocket>);
pub type InboundFramed<TSocket> = Framed<TimeoutStream<upgrade::Negotiated<TSocket>>, InboundCodec>;
type FnAndThen<TSocket> = fn(
    (Option<RPCRequest>, InboundFramed<TSocket>),
) -> FutureResult<InboundOutput<TSocket>, RPCError>;
type FnMapErr<TSocket> = fn(timeout::Error<(RPCError, InboundFramed<TSocket>)>) -> RPCError;

impl<TSocket> InboundUpgrade<TSocket> for RPCProtocol
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Output = InboundOutput<TSocket>;
    type Error = RPCError;

    type Future = future::AndThen<
        future::MapErr<
            timeout::Timeout<stream::StreamFuture<InboundFramed<TSocket>>>,
            FnMapErr<TSocket>,
        >,
        FutureResult<InboundOutput<TSocket>, RPCError>,
        FnAndThen<TSocket>,
    >;

    fn upgrade_inbound(
        self,
        socket: upgrade::Negotiated<TSocket>,
        protocol: ProtocolId,
    ) -> Self::Future {
        match protocol.encoding.as_str() {
            CBOR | _ => {
                let mut timed_socket = TimeoutStream::new(socket);
                timed_socket.set_read_timeout(Some(Duration::from_secs(TTFB_TIMEOUT)));
                Framed::new(timed_socket, InboundCodec)
                    .into_future()
                    .timeout(Duration::from_secs(REQUEST_TIMEOUT))
                    .map_err(RPCError::from as FnMapErr<TSocket>)
                    .and_then({
                        |(req, stream)| match req {
                            Some(req) => futures::future::ok((req, stream)),
                            None => futures::future::err(RPCError::Custom(
                                "Stream terminated early".into(),
                            )),
                        }
                    } as FnAndThen<TSocket>)
            }
        }
    }
}

/* Outbound request */

// Combines all the RPC requests into a single enum to implement `UpgradeInfo` and
// `OutboundUpgrade`

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RPCRequest {
    Status(StatusMessage),
    Goodbye(GoodbyeReason),
    BlockSyncRequest(BlockSyncRequest),
}

impl UpgradeInfo for RPCRequest {
    type Info = ProtocolId;
    type InfoIter = Vec<Self::Info>;

    // add further protocols as we support more encodings/versions
    fn protocol_info(&self) -> Self::InfoIter {
        self.supported_protocols()
    }
}

/// Implements the encoding per supported protocol for RPCRequest.
impl RPCRequest {
    pub fn supported_protocols(&self) -> Vec<ProtocolId> {
        match self {
            // add more protocols when versions/encodings are supported
            RPCRequest::Status(_) => vec![ProtocolId::new(RPC_STATUS, "1", CBOR)],
            RPCRequest::Goodbye(_) => vec![ProtocolId::new(RPC_GOODBYE, "1", CBOR)],
            RPCRequest::BlockSyncRequest(_) => {
                vec![ProtocolId::new(RPC_BLOCK_SYNC_REQUEST, "1", CBOR)]
            }
        }
    }

    /* These functions are used in the handler for stream management */

    /// This specifies whether a stream should remain open and await a response, given a request.
    /// A GOODBYE request has no response.
    pub fn expect_response(&self) -> bool {
        match self {
            RPCRequest::Status(_) => true,
            RPCRequest::Goodbye(_) => false,
            RPCRequest::BlockSyncRequest(_) => true,
        }
    }

    /// Returns which methods expect multiple responses from the stream. If this is false and
    /// the stream terminates, an error is given.
    pub fn multiple_responses(&self) -> bool {
        match self {
            RPCRequest::Status(_) => false,
            RPCRequest::Goodbye(_) => false,
            RPCRequest::BlockSyncRequest(_) => true,
        }
    }

    /// Returns the `ResponseTermination` type associated with the request if a stream gets
    /// terminated.
    pub fn stream_termination(&self) -> ResponseTermination {
        match self {
            // this only gets called after `multiple_responses()` returns true. Therefore, only
            // variants that have `multiple_responses()` can have values.
            RPCRequest::BlockSyncRequest(_) => ResponseTermination::BlockSyncRequest,
            RPCRequest::Status(_) => unreachable!(),
            RPCRequest::Goodbye(_) => unreachable!(),
        }
    }
}

/* RPC Response type - used for outbound upgrades */

/* Outbound upgrades */

pub type OutboundFramed<TSocket> = Framed<upgrade::Negotiated<TSocket>, OutboundCodec>;

impl<TSocket> OutboundUpgrade<TSocket> for RPCRequest
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Output = OutboundFramed<TSocket>;
    type Error = RPCError;
    type Future = sink::Send<OutboundFramed<TSocket>>;
    fn upgrade_outbound(
        self,
        socket: upgrade::Negotiated<TSocket>,
        protocol: Self::Info,
    ) -> Self::Future {
        match protocol.encoding.as_str() {
            CBOR | _ => Framed::new(socket, OutboundCodec).send(self),
        }
    }
}

/// Error in RPC Encoding/Decoding.
#[derive(Debug)]
pub enum RPCError {
    /// Error when reading the packet from the socket.
    ReadError(upgrade::ReadOneError),
    /// Error when decoding the raw buffer from ssz.
    CborDecodeError(serde_cbor::Error),
    /// Invalid Protocol ID.
    InvalidProtocol(&'static str),
    /// IO Error.
    IoError(io::Error),
    /// Waiting for a request/response timed out, or timer error'd.
    StreamTimeout,
    /// The peer returned a valid RPCErrorResponse but the response was an error.
    RPCErrorResponse,
    /// Custom message.
    Custom(String),
}

impl From<upgrade::ReadOneError> for RPCError {
    #[inline]
    fn from(err: upgrade::ReadOneError) -> Self {
        RPCError::ReadError(err)
    }
}

impl From<serde_cbor::Error> for RPCError {
    #[inline]
    fn from(err: serde_cbor::Error) -> Self {
        RPCError::CborDecodeError(err)
    }
}
impl<T> From<tokio::timer::timeout::Error<T>> for RPCError {
    fn from(err: tokio::timer::timeout::Error<T>) -> Self {
        if err.is_elapsed() {
            RPCError::StreamTimeout
        } else {
            RPCError::Custom("Stream timer failed".into())
        }
    }
}

impl From<()> for RPCError {
    fn from(_err: ()) -> Self {
        RPCError::Custom("".into())
    }
}

impl From<io::Error> for RPCError {
    fn from(err: io::Error) -> Self {
        RPCError::IoError(err)
    }
}

// Error trait is required for `ProtocolsHandler`
impl std::fmt::Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RPCError::ReadError(ref err) => write!(f, "Error while reading from socket: {}", err),
            RPCError::CborDecodeError(ref err) => write!(f, "Error while decoding cbor: {:?}", err),
            RPCError::InvalidProtocol(ref err) => write!(f, "Invalid Protocol: {}", err),
            RPCError::IoError(ref err) => write!(f, "IO Error: {}", err),
            RPCError::RPCErrorResponse => write!(f, "RPC Response Error"),
            RPCError::StreamTimeout => write!(f, "Stream Timeout"),
            RPCError::Custom(ref err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for RPCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RPCError::ReadError(ref err) => Some(err),
            RPCError::CborDecodeError(_) => None,
            RPCError::InvalidProtocol(_) => None,
            RPCError::IoError(ref err) => Some(err),
            RPCError::StreamTimeout => None,
            RPCError::RPCErrorResponse => None,
            RPCError::Custom(_) => None,
        }
    }
}

impl std::fmt::Display for RPCRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RPCRequest::Status(status) => write!(f, "Status Message: {}", status),
            RPCRequest::Goodbye(reason) => write!(f, "Goodbye: {}", reason),
            RPCRequest::BlockSyncRequest(req) => write!(f, "Block sync request: {}", req),
        }
    }
}

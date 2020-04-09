// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod blocksync;
mod codec;
mod hello;

pub use self::blocksync::{
    BlockSyncRequest, BlockSyncResponse, BlockSyncTipset, BLOCKSYNC_PROTOCOL_ID,
};
pub use self::codec::{InboundCodec, OutboundCodec};
pub use self::hello::{HelloRequest, HelloResponse, HELLO_PROTOCOL_ID};

use bytes::BytesMut;
use futures::{
    future::BoxFuture,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};
use futures_codec::{Decoder, Encoder, Framed};
use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo};

/// The unique id of RPC request.
pub type RequestId = usize;

/// The type used in the behaviour and the protocols handler.
#[derive(Clone, Debug, PartialEq)]
pub enum RpcMessage {
    /// An inbound/outbound request for RPC protocol.
    /// The first parameter is a sequential id which tracks an awaiting substream for the response.
    Request(RequestId, RpcRequest),
    /// A response that is being sent or has been received from the RPC protocol.
    /// The first parameter returns that which was sent with the corresponding request,
    /// the second is a single chunk of a response.
    Response(RequestId, RpcResponse),
}

impl RpcMessage {
    /// Returns the id which is used to track the substream
    pub fn id(&self) -> usize {
        match *self {
            RpcMessage::Request(id, _) => id,
            RpcMessage::Response(id, _) => id,
        }
    }
}

impl std::fmt::Display for RpcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcMessage::Request(id, request) => {
                write!(f, "RPC Request(id: {:?}, request: {:?})", id, request)
            }
            RpcMessage::Response(id, response) => match response {
                Ok(success_response) => write!(
                    f,
                    "RPC Response(id: {:?}, response: {:?})",
                    id, success_response
                ),
                Err(failure_response) => write!(
                    f,
                    "RPC Response(id: {:?}, response: {:?})",
                    id, failure_response
                ),
            },
        }
    }
}

/// RpcRequest payload
#[derive(Clone, Debug, PartialEq)]
pub enum RpcRequest {
    BlockSync(BlockSyncRequest),
    Hello(HelloRequest),
}

impl RpcRequest {
    pub fn supported_protocol(&self) -> Vec<&'static [u8]> {
        match self {
            RpcRequest::BlockSync(_) => vec![BLOCKSYNC_PROTOCOL_ID],
            RpcRequest::Hello(_) => vec![HELLO_PROTOCOL_ID],
        }
    }

    pub fn expect_response(&self) -> bool {
        match self {
            RpcRequest::BlockSync(_) => true,
            RpcRequest::Hello(_) => true,
        }
    }
}

/// RpcResponse payload
pub type RpcResponse = Result<RpcResponseSuccess, RpcResponseFailure>;

#[derive(Clone, Debug, PartialEq)]
pub enum RpcResponseSuccess {
    BlockSync(BlockSyncResponse),
    Hello(HelloResponse),
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum RpcResponseFailure {
    #[error("Invalid Request: {0}")]
    InvalidRequest(String),
    #[error("Server Error: {0}")]
    ServerError(String),
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum RpcError {
    #[error("Io error: {0}")]
    Io(String),
    #[error("Codec error: {0}")]
    Codec(String),
    #[error("Response failure: {0}")]
    ResponseFailure(String),
    #[error("{0}")]
    Custom(String),
}

impl From<std::io::Error> for RpcError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<serde_cbor::Error> for RpcError {
    fn from(err: serde_cbor::Error) -> Self {
        Self::Codec(err.to_string())
    }
}

/// Protocol upgrade of inbound.
#[derive(Debug, Clone, Copy)]
pub struct InboundProtocol;

impl UpgradeInfo for InboundProtocol {
    type Info = &'static [u8];
    type InfoIter = Vec<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        vec![BLOCKSYNC_PROTOCOL_ID, HELLO_PROTOCOL_ID]
    }
}

impl<TSocket> InboundUpgrade<TSocket> for InboundProtocol
where
    TSocket: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = (RpcRequest, Framed<TSocket, InboundCodec>);
    type Error = RpcError;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, mut socket: TSocket, info: Self::Info) -> Self::Future {
        Box::pin(async move {
            let mut buf = vec![];
            socket.read_to_end(&mut buf).await?;
            let mut codec = InboundCodec::new(info);
            let request = codec.decode(&mut BytesMut::from(&buf[..]))?.unwrap();
            Ok((request, Framed::new(socket, codec)))
        })
    }
}

/// Protocol upgrade of outbound.
pub struct OutboundProtocol(pub(crate) RpcRequest);

impl UpgradeInfo for OutboundProtocol {
    type Info = &'static [u8];
    type InfoIter = Vec<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        self.0.supported_protocol()
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for OutboundProtocol
where
    TSocket: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = Framed<TSocket, OutboundCodec>;
    type Error = RpcError;
    type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, mut socket: TSocket, info: Self::Info) -> Self::Future {
        Box::pin(async move {
            let mut bytes = BytesMut::with_capacity(1024);
            let mut codec = OutboundCodec::new(info);
            codec.encode(self.0, &mut bytes)?;
            socket.write_all(&bytes).await?;
            socket.close().await?;
            Ok(Framed::new(socket, codec))
        })
    }
}

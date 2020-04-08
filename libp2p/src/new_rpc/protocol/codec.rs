// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use bytes::BytesMut;
use futures_codec::{Decoder, Encoder};

use crate::new_rpc::protocol::{
    RpcError, RpcRequest, RpcResponse, RpcResponseSuccess, BLOCKSYNC_PROTOCOL_ID, HELLO_PROTOCOL_ID,
};

/// InboundCodec used for inbound connections.
///
/// recv  ====> -----decode----- ====>  RpcRequest  ====> ---------------
///             | InboundCodec |                          |   handling  |
/// send  <==== -----encode----- <====  RpcResponse <==== ---------------
pub struct InboundCodec {
    protocol: &'static [u8],
}

impl InboundCodec {
    pub fn new(protocol: &'static [u8]) -> Self {
        Self { protocol }
    }
}

impl Encoder for InboundCodec {
    type Item = RpcResponse;
    type Error = RpcError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Ok(RpcResponseSuccess::BlockSync(response)) => {
                let response = serde_cbor::to_vec(&response)?;
                dst.clear();
                dst.extend_from_slice(&response);
                Ok(())
            }
            Ok(RpcResponseSuccess::Hello(response)) => {
                let response = serde_cbor::to_vec(&response)?;
                dst.clear();
                dst.extend_from_slice(&response);
                Ok(())
            }
            Err(failure_rpc_response) => {
                Err(RpcError::ResponseFailure(failure_rpc_response.to_string()))
            }
        }
    }
}

impl Decoder for InboundCodec {
    type Item = RpcRequest;
    type Error = RpcError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        match self.protocol {
            BLOCKSYNC_PROTOCOL_ID => {
                let request = serde_cbor::from_slice(src)?;
                Ok(Some(RpcRequest::BlockSync(request)))
            }
            HELLO_PROTOCOL_ID => {
                let request = serde_cbor::from_slice(src)?;
                Ok(Some(RpcRequest::Hello(request)))
            }
            _ => Err(RpcError::Custom("Unsupported protocol".to_string())),
        }
    }
}

/// OutboundCodec used for outbound connections.
///
/// ---------------  <==== RpcRequest   <==== -----encode------ <====  send
/// |   handling  |                           | OutboundCodec |
/// ---------------  ====> RpcResponse  ====> -----decode------ ====>  recv
pub struct OutboundCodec {
    protocol: &'static [u8],
}

impl OutboundCodec {
    pub fn new(protocol: &'static [u8]) -> Self {
        Self { protocol }
    }
}

impl Encoder for OutboundCodec {
    type Item = RpcRequest;
    type Error = RpcError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            RpcRequest::BlockSync(request) => {
                let request = serde_cbor::to_vec(&request)?;
                dst.clear();
                dst.extend_from_slice(&request);
                Ok(())
            }
            RpcRequest::Hello(request) => {
                let request = serde_cbor::to_vec(&request)?;
                dst.clear();
                dst.extend_from_slice(&request);
                Ok(())
            }
        }
    }
}

impl Decoder for OutboundCodec {
    type Item = RpcResponse;
    type Error = RpcError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        match self.protocol {
            BLOCKSYNC_PROTOCOL_ID => {
                let response = serde_cbor::from_slice(src)?;
                Ok(Some(RpcResponse::Ok(RpcResponseSuccess::BlockSync(
                    response,
                ))))
            }
            HELLO_PROTOCOL_ID => {
                let response = serde_cbor::from_slice(src)?;
                Ok(Some(RpcResponse::Ok(RpcResponseSuccess::Hello(response))))
            }
            _ => Err(RpcError::Custom("Unsupported protocol".to_string())),
        }
    }
}

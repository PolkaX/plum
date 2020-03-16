// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use libp2p::bytes::BytesMut;
use tokio::codec::{Decoder, Encoder};

use crate::rpc::protocol::RPCError;
use crate::rpc::{RPCErrorResponse, RPCRequest};

fn encode_to<T: serde::Serialize>(item: T, dst: &mut BytesMut) -> Result<(), serde_cbor::Error> {
    let encoded = serde_cbor::to_vec(&item)?;
    dst.clear();
    dst.extend_from_slice(&encoded);
    Ok(())
}

pub struct InboundCodec;

impl Encoder for InboundCodec {
    type Item = RPCErrorResponse;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        encode_to(item, dst).map_err(Into::into)
    }
}

impl Decoder for InboundCodec {
    type Item = RPCRequest;
    type Error = RPCError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let value: Self::Item = serde_cbor::from_slice(src)?;
        Ok(Some(value))
    }
}

pub struct OutboundCodec;

impl Encoder for OutboundCodec {
    type Item = RPCRequest;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        encode_to(item, dst).map_err(Into::into)
    }
}

impl Decoder for OutboundCodec {
    type Item = RPCErrorResponse;
    type Error = RPCError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let value: Self::Item = serde_cbor::from_slice(src)?;
        Ok(Some(value))
    }
}

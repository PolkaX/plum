use libp2p::bytes::{BufMut, BytesMut};
use tokio::codec::{Decoder, Encoder};

use crate::rpc::protocol::RPCError;
use crate::rpc::{RPCErrorResponse, RPCRequest};

pub struct InboundCodec;

impl Encoder for InboundCodec {
    type Item = RPCErrorResponse;
    type Error = RPCError;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let encoded = serde_cbor::to_vec(&item)?;
        // TODO: opotimize?
        for u in encoded {
            dst.put(u);
        }
        Ok(())
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
        let encoded = serde_cbor::to_vec(&item)?;
        for u in encoded {
            dst.put(u);
        }
        Ok(())
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

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::io;

use cid::Cid;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use libp2p::request_response::{ProtocolName, RequestResponseCodec};
use minicbor::{decode, encode, Decoder, Encoder};

use plum_bigint::{BigInt, BigIntRefWrapper};
use plum_block::BlockHeader;
use plum_message::{SignedMessage, UnsignedMessage};

use super::other_io_error;

/// The protocol ID of blocksync.
pub const BLOCKSYNC_PROTOCOL_ID: &[u8] = b"/fil/sync/blk/0.0.1";

/// The protocol name of blocksync protocol.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockSyncProtocolName;

impl ProtocolName for BlockSyncProtocolName {
    fn protocol_name(&self) -> &[u8] {
        BLOCKSYNC_PROTOCOL_ID
    }
}

/// The blocksync request.
///
/// See lotus/chain/blocksync/blocksync.go for details.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSyncRequest {
    ///
    pub start: Vec<Cid>,
    ///
    pub request_length: u64,
    ///
    pub options: u64,
}

// Implement CBOR serialization for BlockSyncRequest.
impl encode::Encode for BlockSyncRequest {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.start)?
            .u64(self.request_length)?
            .u64(self.options)?
            .ok()
    }
}

// Implement CBOR deserialization for BlockSyncRequest  .
impl<'b> decode::Decode<'b> for BlockSyncRequest {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(Self {
            start: d.decode()?,
            request_length: d.decode()?,
            options: d.decode()?,
        })
    }
}

/// The response to a blocksync request.
///
/// See lotus/chain/blocksync/blocksync.go for details.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSyncResponse {
    /// The tipsets requested
    pub chain: Vec<BlockSyncTipset>,
    /// Error code
    pub status: u64,
    /// Status message indicating failure reason
    pub message: String,
}

// Implement CBOR serialization for BlockSyncResponse.
impl encode::Encode for BlockSyncResponse {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.chain)?
            .u64(self.status)?
            .str(&self.message)?
            .ok()
    }
}

// Implement CBOR deserialization for BlockSyncResponse.
impl<'b> decode::Decode<'b> for BlockSyncResponse {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(Self {
            chain: d.decode()?,
            status: d.u64()?,
            message: d.str()?.to_owned(),
        })
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSyncTipset {
    /// The blocks in the tipset
    pub blocks: Vec<BlockHeader>,

    /// Signed bls messages
    pub bls_msgs: Vec<UnsignedMessage>,
    /// Describes which block each message belongs to
    pub bls_msg_includes: Vec<Vec<u64>>,

    /// Unsigned secp messages
    pub secp_msgs: Vec<SignedMessage>,
    /// Describes which block each message belongs to
    pub secp_msg_includes: Vec<Vec<u64>>,
}

// Implement CBOR serialization for BlockSyncTipset.
impl encode::Encode for BlockSyncTipset {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(5)?
            .encode(&self.blocks)?
            .encode(&self.bls_msgs)?
            .encode(&self.bls_msg_includes)?
            .encode(&self.secp_msgs)?
            .encode(&self.secp_msg_includes)?
            .ok()
    }
}

// Implement CBOR deserialization for BlockSyncTipset.
impl<'b> decode::Decode<'b> for BlockSyncTipset {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(5));
        Ok(Self {
            blocks: d.decode()?,
            bls_msgs: d.decode()?,
            bls_msg_includes: d.decode()?,
            secp_msgs: d.decode()?,
            secp_msg_includes: d.decode()?
        })
    }
}

/// The codec to be used for blocksync protocol.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockSyncCodec;

#[async_trait::async_trait]
impl RequestResponseCodec for BlockSyncCodec {
    type Protocol = BlockSyncProtocolName;
    type Request = BlockSyncRequest;
    type Response = BlockSyncResponse;

    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut request = Vec::new();
        io.read_to_end(&mut request).await?;
        minicbor::decode(&request).map_err(|e| other_io_error(e.to_string()))
    }

    async fn read_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut response = Vec::new();
        io.read_to_end(&mut response).await?;
        minicbor::decode(&response).map_err(|e| other_io_error(e.to_string()))
    }

    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let request = minicbor::to_vec(req).map_err(|e| other_io_error(e.to_string()))?;
        io.write_all(&request).await
    }

    async fn write_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let response = minicbor::to_vec(res).map_err(|e| other_io_error(e.to_string()))?;
        io.write_all(&response).await
    }
}

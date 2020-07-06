// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::io;

use cid::Cid;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use libp2p::request_response::{ProtocolName, RequestResponseCodec};
use minicbor::{decode, encode, Decoder, Encoder};

use plum_bigint::{BigInt, BigIntRefWrapper, BigIntWrapper};
use plum_types::ChainEpoch;

use super::other_io_error;

/// The protocol ID of hello.
pub const HELLO_PROTOCOL_ID: &[u8] = b"/fil/hello/1.0.0";

/// The protocol name of hello protocol.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HelloProtocolName;

impl ProtocolName for HelloProtocolName {
    fn protocol_name(&self) -> &[u8] {
        HELLO_PROTOCOL_ID
    }
}

/// The Hello request.
///
/// See https://filecoin-project.github.io/specs/#hello-spec or lotus/node/hello/hello.go for details.
#[derive(Clone, Debug, PartialEq)]
pub struct HelloRequest {
    ///
    pub heaviest_tip_set: Vec<Cid>,
    ///
    pub heaviest_tipset_height: ChainEpoch,
    ///
    pub heaviest_tipset_weight: BigInt,
    ///
    pub genesis_hash: Cid,
}

// Implement CBOR serialization for HelloRequest.
impl encode::Encode for HelloRequest {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(4)?
            .encode(&self.heaviest_tip_set)?
            .encode(&self.heaviest_tipset_height)?
            .encode(&BigIntRefWrapper::from(&self.heaviest_tipset_weight))?
            .encode(&self.genesis_hash)?
            .ok()
    }
}

// Implement CBOR deserialization for HelloRequest.
impl<'b> decode::Decode<'b> for HelloRequest {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(4));
        Ok(Self {
            heaviest_tip_set: d.decode()?,
            heaviest_tipset_height: d.decode()?,
            heaviest_tipset_weight: d.decode::<BigIntWrapper>()?.into_inner(),
            genesis_hash: d.decode()?,
        })
    }
}

/// The response to a Hello request.
///
/// See https://filecoin-project.github.io/specs/#hello-spec or lotus/node/hello/hello.go for details.
#[derive(Clone, Debug, PartialEq)]
pub struct HelloResponse {
    /// Time of arrival in unix nanoseconds
    pub arrival: i64,
    /// Time sent in unix nanoseconds
    pub sent: i64,
}

// Implement CBOR serialization for HelloResponse.
impl encode::Encode for HelloResponse {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?.i64(self.arrival)?.i64(self.sent)?.ok()
    }
}

// Implement CBOR deserialization for HelloResponse.
impl<'b> decode::Decode<'b> for HelloResponse {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(Self {
            arrival: d.i64()?,
            sent: d.i64()?,
        })
    }
}

/// The codec to be used for hello protocol.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HelloCodec;

#[async_trait::async_trait]
impl RequestResponseCodec for HelloCodec {
    type Protocol = HelloProtocolName;
    type Request = HelloRequest;
    type Response = HelloResponse;

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

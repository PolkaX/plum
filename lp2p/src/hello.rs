// Copyright 2019 PolkaX

use chain::types::{BigInt, Cid};
use futures::future::{self, FutureResult};
use futures::prelude::*;
use libp2p::core::{
    upgrade::Negotiated, ConnectedPoint, InboundUpgrade, Multiaddr, OutboundUpgrade, PeerId,
    UpgradeInfo,
};
use libp2p::swarm::{
    KeepAlive, NetworkBehaviour, NetworkBehaviourAction, PollParameters, ProtocolsHandler,
    ProtocolsHandlerEvent, ProtocolsHandlerUpgrErr, SubstreamProtocol,
};
use libp2p::tokio_io::{AsyncRead, AsyncWrite};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
struct HelloMsg {
    HeaviestTipSet: Vec<Cid>,
    HeaviestTipSetWeight: BigInt,
    GenesisHash: Cid,
}

pub struct Hello<TSubstream> {
    GenesisHash: Cid,
    _marker: PhantomData<TSubstream>,
}

impl<TSubstream> Hello<TSubstream> {
    pub fn new(genesis_hash: Cid) -> Self {
        Hello {
            GenesisHash: genesis_hash,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TSubstream> NetworkBehaviour for Hello<TSubstream>
where
    TSubstream: AsyncRead + AsyncWrite,
{
    type ProtocolsHandler = HelloHandler<TSubstream>;
    type OutEvent = HelloEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        HelloHandler::new()
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: PeerId, endpoint: ConnectedPoint) {
        // Say hello
    }

    fn inject_disconnected(&mut self, _: &PeerId, _: ConnectedPoint) {}

    fn inject_node_event(
        &mut self,
        _: PeerId,
        _: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
    }

    fn poll(
        &mut self,
        params: &mut impl PollParameters,
    ) -> Async<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        Async::NotReady
    }
}

pub struct HelloHandler<TSubstream> {
    _marker: std::marker::PhantomData<TSubstream>,
}

impl<TSubstream> HelloHandler<TSubstream> {
    pub fn new() -> Self {
        HelloHandler {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TSubstream> ProtocolsHandler for HelloHandler<TSubstream>
where
    TSubstream: AsyncRead + AsyncWrite,
{
    type InEvent = ();
    type OutEvent = ();
    type Error = std::io::Error;
    type Substream = TSubstream;
    type InboundProtocol = HelloProtocol;
    type OutboundProtocol = HelloProtocol;
    type OutboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<HelloProtocol> {
        SubstreamProtocol::new(HelloProtocol)
    }

    fn inject_fully_negotiated_inbound(&mut self, _: ()) {}

    fn inject_fully_negotiated_outbound(&mut self, _: (), _: ()) {}

    fn inject_event(&mut self, _: ()) {}

    fn inject_dial_upgrade_error(&mut self, _: (), _: ProtocolsHandlerUpgrErr<std::io::Error>) {}

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::Yes
    }

    fn poll(&mut self) -> Poll<ProtocolsHandlerEvent<HelloProtocol, (), ()>, Self::Error> {
        Ok(Async::NotReady)
    }
}

#[derive(Debug)]
pub enum HelloEvent {}

#[derive(Default, Debug, Copy, Clone)]
pub struct HelloProtocol;

impl UpgradeInfo for HelloProtocol {
    type Info = &'static [u8];
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once(b"/filecoin/hello/1.0.0")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for HelloProtocol
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Output = ();
    type Error = std::io::Error;
    type Future = FutureResult<Self::Output, Self::Error>;

    fn upgrade_inbound(self, _: Negotiated<TSocket>, _: Self::Info) -> Self::Future {
        future::ok(())
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for HelloProtocol
where
    TSocket: AsyncRead + AsyncWrite,
{
    type Output = ();
    type Error = std::io::Error;
    type Future = FutureResult<Self::Output, Self::Error>;

    fn upgrade_outbound(self, w: Negotiated<TSocket>, _: Self::Info) -> Self::Future {
        future::ok(())
    }
}

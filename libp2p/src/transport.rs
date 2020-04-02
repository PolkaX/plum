// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::io;
use std::time::Duration;

use libp2p::{
    core::{
        identity,
        muxing::StreamMuxerBox,
        transport::{boxed::Boxed, Transport},
        upgrade, PeerId,
    },
    dns, mplex, secio, tcp, yamux,
};

/// Builds the transport that serves as a common ground for all connections.
pub fn build_transport(local_key: identity::Keypair) -> Boxed<(PeerId, StreamMuxerBox), io::Error> {
    let secio_config = secio::SecioConfig::new(local_key);
    let yamux_config = yamux::Config::default();
    let mplex_config = mplex::MplexConfig::new();

    let transport = tcp::TcpConfig::new().nodelay(true);
    let transport = dns::DnsConfig::new(transport).expect("Create the DNS config");

    transport
        .upgrade(upgrade::Version::V1)
        .authenticate(secio_config)
        .multiplex(upgrade::SelectUpgrade::new(yamux_config, mplex_config))
        .map(|(peer_id, muxer), _endpoint| (peer_id, StreamMuxerBox::new(muxer)))
        .timeout(Duration::from_secs(20))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        .boxed()
}

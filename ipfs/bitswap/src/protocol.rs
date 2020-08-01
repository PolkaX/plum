// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use libp2p::request_response::ProtocolName;

/// The protocol ID of IPFS bitswap.
pub const BITSWAP_PROTOCOL_ID: &[u8] = b"/ipfs/bitswap/1.1.0";

/// The protocol name of IPFS bitswap protocol.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitswapProtocolName;

impl ProtocolName for BitswapProtocolName {
    fn protocol_name(&self) -> &[u8] {
        BITSWAP_PROTOCOL_ID
    }
}

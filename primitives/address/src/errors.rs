// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use thiserror::Error;

use crate::network::NETWORK_DEFAULT;

/// Errors generated from this library.
#[derive(PartialEq, Eq, Debug, Error)]
pub enum AddressError {
    /// Unknown network.
    #[error("unknown network")]
    UnknownNetwork,
    /// Mismatch network.
    #[error("Network do not match default network (current: {})", NETWORK_DEFAULT.prefix())]
    MismatchNetwork,
    /// Unknown address protocol.
    #[error("unknown protocol")]
    UnknownProtocol,
    /// Invalid address payload.
    #[error("invalid address payload")]
    InvalidPayload,
    /// Invalid address length.
    #[error("invalid address length")]
    InvalidLength,
    /// Invalid address checksum.
    #[error("invalid address checksum")]
    InvalidChecksum,
    /// Base32 decode error.
    #[error("base32 decode error: {0}")]
    Base32Decode(#[from] data_encoding::DecodeError),
}

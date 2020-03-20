// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use thiserror::Error;

use crate::network::NETWORK_DEFAULT;

/// Errors generated from this library.
#[derive(PartialEq, Eq, Debug, Error)]
pub enum AddressError {
    /// Unknown network.
    #[error("Unknown network")]
    UnknownNetwork,
    /// Mismatch network.
    #[error("Network do not match default network. current:{}", NETWORK_DEFAULT.prefix())]
    MismatchNetwork,
    /// Unknown address protocol.
    #[error("Unknown protocol")]
    UnknownProtocol,
    /// Unknown public key type.
    #[error("Unknown public key type")]
    UnknownPublicKey,
    /// Invalid address payload.
    #[error("Invalid address payload")]
    InvalidPayload,
    /// Invalid address length.
    #[error("Invalid address length")]
    InvalidLength,
    /// Invalid address checksum.
    #[error("Invalid address checksum")]
    InvalidChecksum,
    /// Base32 decode error.
    #[error("Base32 decode error: {0}")]
    Base32Decode(#[from] data_encoding::DecodeError),
}

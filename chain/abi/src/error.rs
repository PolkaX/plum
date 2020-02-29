// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use thiserror::Error;

/// Errors generated from this library.
#[derive(PartialEq, Eq, Debug, Error)]
pub enum AbiError {
    /// Unknown network.
    #[error("Unsupported proof type")]
    Unsupported,
    /// Unknown address protocol.
    #[error("Unknown protocol")]
    UnknownProtocol,
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

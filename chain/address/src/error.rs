// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// Type alias to use this library's [`AddressError`] type in a `Result`.
pub type Result<T> = std::result::Result<T, AddressError>;

/// Errors generated from this library.
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum AddressError {
    /// Unknown network.
    #[error("Unknown network")]
    UnknownNetwork,
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
    #[error("Base32 decode error")]
    Base32Decode(#[from] data_encoding::DecodeError),
}

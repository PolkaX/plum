// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// Alias for a `Result` with the default error type `IpldError`.
pub type Result<T, E = IpldError> = std::result::Result<T, E>;

/// The IPLD error.
#[derive(Debug, thiserror::Error)]
pub enum IpldError {
    /// IO error.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// CBOR decode error.
    #[error("{0}")]
    CborDecode(#[from] minicbor::decode::Error),
    /// JSON Codec error.
    #[error("{0}")]
    JsonCodec(#[from] serde_json::Error),
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// The IPLD error.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum IpldError {
    /// Codec error.
    #[error("{0}")]
    Codec(String),
    /// Custom error.
    #[error("{0}")]
    Custom(String),
}

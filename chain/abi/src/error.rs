// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use thiserror::Error;

/// Errors generated from this library.
#[derive(PartialEq, Eq, Debug, Error)]
pub enum AbiError {
    /// Unknown network.
    #[error("Unsupported proof type")]
    Unsupported,
    /// Size too small.
    #[error("Size too small, minimum piece size is 127 bytes")]
    SizeTooSmall,
    /// Invalid unpadded piece size or padded piece size.
    #[error("Invalid unpadded piece size or padded piece size")]
    InvalidSize,
}

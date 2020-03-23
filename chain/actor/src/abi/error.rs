// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use thiserror::Error;

pub type PieceResult<T> = std::result::Result<T, PieceError>;

/// Piece Errors
#[derive(PartialEq, Eq, Debug, Error)]
pub enum PieceError {
    /// Size too small.
    #[error("Size too small, minimum piece size is 127 bytes, current:{0}")]
    SizeTooSmall(u64),
    /// Invalid unpadded piece size.
    #[error("Unpadded piece size must be a power of 2 multiple of 127, current:{0}")]
    UnpadedSizeError(u64),
    /// Invalid Padded piece size.
    #[error("Padded piece size must be a power of 2, current:{0}")]
    PadedSizeError(u64),
}

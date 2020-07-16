// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

const MIN_UNPADDED_PIECE_SIZE: u64 = 127;
const MIN_PADDED_PIECE_SIZE: u64 = 128;

/// Unpadded size of a piece, in bytes
#[derive(
    Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub struct UnpaddedPieceSize(pub u64);

impl UnpaddedPieceSize {
    /// Convert unpadded piece size into padded piece size.
    pub fn padded(self) -> PaddedPieceSize {
        PaddedPieceSize(self.0 + (self.0 / MIN_UNPADDED_PIECE_SIZE))
    }

    /// Validate that whether the UnpaddedPieceSize is valid.
    pub fn validate(self) -> Result<(), PieceSizeError> {
        if self.0 < MIN_UNPADDED_PIECE_SIZE {
            return Err(PieceSizeError::UnpaddedSizeTooSmall(self.0));
        }

        // unpadded piece size must be a power of 2 multiple of 127 (127 * 2^n)
        if self.0 >> self.0.trailing_zeros() != MIN_UNPADDED_PIECE_SIZE {
            return Err(PieceSizeError::InvalidUnpaddedSize(self.0));
        }

        Ok(())
    }
}

/// Padded size of a piece, in bytes.
#[derive(
    Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub struct PaddedPieceSize(pub u64);

impl PaddedPieceSize {
    /// Convert padded piece size into unpadded piece size.
    pub fn unpadded(self) -> UnpaddedPieceSize {
        UnpaddedPieceSize(self.0 - (self.0 / MIN_PADDED_PIECE_SIZE))
    }

    /// Validate that whether the PaddedPieceSize is valid.
    pub fn validate(self) -> Result<(), PieceSizeError> {
        if self.0 < MIN_PADDED_PIECE_SIZE {
            return Err(PieceSizeError::PaddedSizeTooSmall(self.0));
        }

        // padded piece size must be a power of 2 (2^n)
        if self.0.count_ones() != 1 {
            return Err(PieceSizeError::InvalidPaddedSize(self.0));
        }

        Ok(())
    }
}

/// THe Errors of validating the piece size.
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum PieceSizeError {
    /// Unpadded piece size is too small.
    #[error("minimum unpadded piece size is 127 bytes, current:{0}")]
    UnpaddedSizeTooSmall(u64),
    /// Invalid unpadded piece size.
    #[error("unpadded piece size must be a power of 2 multiple of 127, current:{0}")]
    InvalidUnpaddedSize(u64),
    /// Padded size is too small.
    #[error("minimum padded piece size is 128 bytes, current:{0}")]
    PaddedSizeTooSmall(u64),
    /// Invalid padded piece size.
    #[error("padded piece size must be a power of 2, current:{0}")]
    InvalidPaddedSize(u64),
}

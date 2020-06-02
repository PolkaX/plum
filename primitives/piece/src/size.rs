// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::ops::{Add, Deref, DerefMut, Sub};

use serde::{Deserialize, Serialize};

const MIN_UNPADDED_PIECE_SIZE: u64 = 127;
const MIN_PADDED_PIECE_SIZE: u64 = 128;

/// Unpadded size of a piece, in bytes
#[derive(
    Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub struct UnpaddedPieceSize(pub(crate) u64);

impl From<UnpaddedPieceSize> for u64 {
    fn from(size: UnpaddedPieceSize) -> Self {
        size.0
    }
}

impl Into<UnpaddedPieceSize> for u64 {
    fn into(self) -> UnpaddedPieceSize {
        UnpaddedPieceSize::new(self)
    }
}

impl Add for UnpaddedPieceSize {
    type Output = UnpaddedPieceSize;

    fn add(self, other: UnpaddedPieceSize) -> UnpaddedPieceSize {
        UnpaddedPieceSize(self.0 + other.0)
    }
}

impl Sub for UnpaddedPieceSize {
    type Output = UnpaddedPieceSize;

    fn sub(self, other: UnpaddedPieceSize) -> UnpaddedPieceSize {
        UnpaddedPieceSize(self.0 - other.0)
    }
}

impl Deref for UnpaddedPieceSize {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UnpaddedPieceSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UnpaddedPieceSize {
    /// Create an unpadded piece size with the given `size`.
    pub fn new(size: u64) -> Self {
        UnpaddedPieceSize(size)
    }

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
pub struct PaddedPieceSize(pub(crate) u64);

impl From<PaddedPieceSize> for u64 {
    fn from(size: PaddedPieceSize) -> Self {
        size.0
    }
}

impl Into<PaddedPieceSize> for u64 {
    fn into(self) -> PaddedPieceSize {
        PaddedPieceSize::new(self)
    }
}

impl Add for PaddedPieceSize {
    type Output = PaddedPieceSize;

    fn add(self, other: PaddedPieceSize) -> PaddedPieceSize {
        PaddedPieceSize(self.0 + other.0)
    }
}

impl Sub for PaddedPieceSize {
    type Output = PaddedPieceSize;

    fn sub(self, other: PaddedPieceSize) -> PaddedPieceSize {
        PaddedPieceSize(self.0 - other.0)
    }
}

impl Deref for PaddedPieceSize {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PaddedPieceSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PaddedPieceSize {
    /// Create an padded piece size with the given `size`.
    pub fn new(size: u64) -> Self {
        PaddedPieceSize(size)
    }

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

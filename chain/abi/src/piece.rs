use crate::error::AbiError;
use cid::Cid;

// UnpaddedPieceSize is the size of a piece, in bytes
#[derive(Debug, Clone)]
pub struct UnpaddedPieceSize(u64);

#[derive(Debug, Clone)]
pub struct PaddedPieceSize(u64);

const MIN_UNPADDED_PIECE_SIZE: u64 = 127;
const MIN_PADDED_PIECE_SIZE: u64 = 128;

impl UnpaddedPieceSize {
    pub fn padded(&self) -> PaddedPieceSize {
        PaddedPieceSize(self.0 + (self.0 / MIN_UNPADDED_PIECE_SIZE))
    }

    pub fn validate(&self) -> std::result::Result<(), AbiError> {
        if self.0 < MIN_UNPADDED_PIECE_SIZE {
            return Err(AbiError::SizeTooSmall) // minimum piece size is 127 bytes
        }
        // is 127 * 2^n
        if self.0 >> self.0.trailing_zeros() != MIN_UNPADDED_PIECE_SIZE {
            return Err(AbiError::InvalidSize) //unpadded piece size must be a power of 2 multiple of 127
        }
        Ok(())
    }
}

impl PaddedPieceSize {
    pub fn unpadded(&self) -> UnpaddedPieceSize {
        UnpaddedPieceSize(self.0 - (self.0 / MIN_PADDED_PIECE_SIZE))
    }

    pub fn validate(&self) -> std::result::Result<(), AbiError> {
        if self.0 < MIN_PADDED_PIECE_SIZE {
            return Err(AbiError::SizeTooSmall)
        }

        if self.0.count_ones() != 1 {
            return Err(AbiError::InvalidSize) //padded piece size must be a power of 2
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PieceInfo {
    size: PaddedPieceSize, // Size in nodes. For BLS12-381 (capacity 254 bits), must be >= 16. (16 * 8 = 128)
    piece_cid: Cid
}
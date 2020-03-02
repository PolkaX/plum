use crate::error::AbiError;
use cid::Cid;

// UnpaddedPieceSize is the size of a piece, in bytes
#[derive(Debug, Clone, PartialEq)]
pub struct UnpaddedPieceSize(u64);

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validate() {
        assert_eq!(UnpaddedPieceSize(127).validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(1016).validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(34091302912).validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(254).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(255).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(128).validate(), Ok(()));

        assert_eq!(PaddedPieceSize(128).validate(), Ok(()));
        assert_eq!(PaddedPieceSize(1024).validate(), Ok(()));
        assert_eq!(PaddedPieceSize(34359738368).validate(), Ok(()));
        assert_eq!(PaddedPieceSize(512).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(126).validate(), Ok(()));

        assert_eq!(PaddedPieceSize(128), UnpaddedPieceSize(127).padded());
        assert_eq!(PaddedPieceSize(1024), UnpaddedPieceSize(1016).padded());
        assert_eq!(PaddedPieceSize(34359738368), UnpaddedPieceSize(34091302912).padded());
        assert_eq!(UnpaddedPieceSize(127), PaddedPieceSize(128).unpadded());
        assert_eq!(UnpaddedPieceSize(1016), PaddedPieceSize(1024).unpadded());
        assert_eq!(UnpaddedPieceSize(34091302912), PaddedPieceSize(34359738368).unpadded());

        assert_eq!(UnpaddedPieceSize(127).padded().validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(1016).padded().validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(34091302912).padded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(128).unpadded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(1024).unpadded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(34359738368).unpadded().validate(), Ok(()));

        assert_eq!(UnpaddedPieceSize(127).padded().unpadded().validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(1016).padded().unpadded().validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(34091302912).padded().unpadded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(128).unpadded().padded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(1024).unpadded().padded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(34359738368).unpadded().padded().validate(), Ok(()));

        assert_ne!(UnpaddedPieceSize(9).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(128).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(99453687).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(1016+0x1000000).validate(), Ok(()));

        assert_ne!(PaddedPieceSize(8).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(127).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(99453687).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(0xc00).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(1025).validate(), Ok(()));

    }
}
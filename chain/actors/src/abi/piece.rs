use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use cid::Cid;

use super::error::{PieceError, PieceResult};

// UnpaddedPieceSize is the size of a piece, in bytes
#[derive(Debug, Clone, PartialEq)]
pub struct UnpaddedPieceSize(u64);

impl UnpaddedPieceSize {
    pub fn new(u: u64) -> PieceResult<Self> {
        let p = UnpaddedPieceSize(u);
        p.validate()?;
        Ok(p)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaddedPieceSize(u64);

impl PaddedPieceSize {
    pub fn new(u: u64) -> PieceResult<Self> {
        let p = PaddedPieceSize(u);
        p.validate()?;
        Ok(p)
    }
}

const MIN_UNPADDED_PIECE_SIZE: u64 = 127;
const MIN_PADDED_PIECE_SIZE: u64 = 128;

impl UnpaddedPieceSize {
    pub fn padded(&self) -> PaddedPieceSize {
        PaddedPieceSize(self.0 + (self.0 / MIN_UNPADDED_PIECE_SIZE))
    }

    pub fn validate(&self) -> PieceResult<()> {
        if self.0 < MIN_UNPADDED_PIECE_SIZE {
            return Err(PieceError::SizeTooSmall(self.0)); // minimum piece size is 127 bytes
        }
        // is 127 * 2^n
        if self.0 >> self.0.trailing_zeros() != MIN_UNPADDED_PIECE_SIZE {
            // unpadded piece size must be a power of 2 multiple of 127
            return Err(PieceError::UnpadedSizeError(self.0));
        }
        Ok(())
    }
}

impl PaddedPieceSize {
    pub fn unpadded(&self) -> UnpaddedPieceSize {
        UnpaddedPieceSize(self.0 - (self.0 / MIN_PADDED_PIECE_SIZE))
    }

    pub fn validate(&self) -> PieceResult<()> {
        if self.0 < MIN_PADDED_PIECE_SIZE {
            return Err(PieceError::SizeTooSmall(self.0));
        }

        if self.0.count_ones() != 1 {
            //padded piece size must be a power of 2
            return Err(PieceError::PadedSizeError(self.0));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize_tuple, Deserialize_tuple)]
pub struct PieceInfo {
    pub size: PaddedPieceSize, // Size in nodes. For BLS12-381 (capacity 254 bits), must be >= 16. (16 * 8 = 128)
    pub piece_cid: Cid,
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
        assert_eq!(
            PaddedPieceSize(34359738368),
            UnpaddedPieceSize(34091302912).padded()
        );
        assert_eq!(UnpaddedPieceSize(127), PaddedPieceSize(128).unpadded());
        assert_eq!(UnpaddedPieceSize(1016), PaddedPieceSize(1024).unpadded());
        assert_eq!(
            UnpaddedPieceSize(34091302912),
            PaddedPieceSize(34359738368).unpadded()
        );

        assert_eq!(UnpaddedPieceSize(127).padded().validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(1016).padded().validate(), Ok(()));
        assert_eq!(UnpaddedPieceSize(34091302912).padded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(128).unpadded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(1024).unpadded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(34359738368).unpadded().validate(), Ok(()));

        assert_eq!(
            UnpaddedPieceSize(127).padded().unpadded().validate(),
            Ok(())
        );
        assert_eq!(
            UnpaddedPieceSize(1016).padded().unpadded().validate(),
            Ok(())
        );
        assert_eq!(
            UnpaddedPieceSize(34091302912)
                .padded()
                .unpadded()
                .validate(),
            Ok(())
        );
        assert_eq!(PaddedPieceSize(128).unpadded().padded().validate(), Ok(()));
        assert_eq!(PaddedPieceSize(1024).unpadded().padded().validate(), Ok(()));
        assert_eq!(
            PaddedPieceSize(34359738368).unpadded().padded().validate(),
            Ok(())
        );

        assert_ne!(UnpaddedPieceSize(9).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(128).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(99453687).validate(), Ok(()));
        assert_ne!(UnpaddedPieceSize(1016 + 0x1000000).validate(), Ok(()));

        assert_ne!(PaddedPieceSize(8).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(127).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(99453687).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(0xc00).validate(), Ok(()));
        assert_ne!(PaddedPieceSize(1025).validate(), Ok(()));
    }

    #[test]
    fn test_cbor() {
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        let info = PieceInfo {
            size: PaddedPieceSize::new(128).unwrap(),
            piece_cid: cid,
        };

        let v = serde_cbor::to_vec(&info).unwrap();
        assert_eq!(
            v,
            vec![
                130, 24, 128, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97,
                161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217,
                73, 55, 160, 199, 184, 78, 250
            ]
        );

        let expect: PieceInfo = serde_cbor::from_slice(&v).unwrap();
        assert_eq!(info, expect);
    }
}

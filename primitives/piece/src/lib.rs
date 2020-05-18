// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! PaddedPieceSize, UnpaddedPieceSize and PieceInfo with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod piece;
mod size;

pub use self::piece::PieceInfo;
pub use self::size::{PaddedPieceSize, PieceSizeError, UnpaddedPieceSize};

#[cfg(test)]
mod tests {
    use cid::Cid;

    use super::*;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    #[rustfmt::skip]
    fn test_piece_size_validate() {
        // happy
        assert!(UnpaddedPieceSize(127).validate().is_ok());
        assert!(UnpaddedPieceSize(1016).validate().is_ok());
        assert!(UnpaddedPieceSize(34_091_302_912).validate().is_ok());

        assert!(PaddedPieceSize(128).validate().is_ok());
        assert!(PaddedPieceSize(1024).validate().is_ok());
        assert!(PaddedPieceSize(34_359_738_368).validate().is_ok());

        // unhappy
        assert!(UnpaddedPieceSize(9).validate().is_err());
        assert!(UnpaddedPieceSize(128).validate().is_err());
        assert!(UnpaddedPieceSize(99_453_687).validate().is_err());
        assert!(UnpaddedPieceSize(1016 + 0x1_000_000).validate().is_err());

        assert!(PaddedPieceSize(8).validate().is_err());
        assert!(PaddedPieceSize(127).validate().is_err());
        assert!(PaddedPieceSize(99_453_687).validate().is_err());
        assert!(PaddedPieceSize(0xc00).validate().is_err());
        assert!(PaddedPieceSize(1025).validate().is_err());

        // convert
        assert_eq!(PaddedPieceSize(128), UnpaddedPieceSize(127).padded());
        assert_eq!(PaddedPieceSize(1024), UnpaddedPieceSize(1016).padded());
        assert_eq!(PaddedPieceSize(34_359_738_368), UnpaddedPieceSize(34_091_302_912).padded());

        assert_eq!(UnpaddedPieceSize(127), PaddedPieceSize(128).unpadded());
        assert_eq!(UnpaddedPieceSize(1016), PaddedPieceSize(1024).unpadded());
        assert_eq!(UnpaddedPieceSize(34_091_302_912), PaddedPieceSize(34_359_738_368).unpadded());

        // swap
        assert!(UnpaddedPieceSize(127).padded().validate().is_ok());
        assert!(UnpaddedPieceSize(1016).padded().validate().is_ok());
        assert!(UnpaddedPieceSize(34_091_302_912).padded().validate().is_ok());

        assert!(PaddedPieceSize(128).unpadded().validate().is_ok());
        assert!(PaddedPieceSize(1024).unpadded().validate().is_ok());
        assert!(PaddedPieceSize(34_359_738_368).unpadded().validate().is_ok());

        // roundtrip
        assert!(UnpaddedPieceSize(127).padded().unpadded().validate().is_ok());
        assert!(UnpaddedPieceSize(1016).padded().unpadded().validate().is_ok());
        assert!(UnpaddedPieceSize(34_091_302_912).padded().unpadded().validate().is_ok());

        assert!(PaddedPieceSize(128).unpadded().padded().validate().is_ok());
        assert!(PaddedPieceSize(1024).unpadded().padded().validate().is_ok());
        assert!(PaddedPieceSize(34_359_738_368).unpadded().padded().validate().is_ok());
    }

    #[test]
    fn test_piece_info_cbor_serde() {
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        let info = PieceInfo {
            size: PaddedPieceSize::new(128).unwrap(),
            piece_cid: cid,
        };
        let expected = vec![
            130, 24, 128, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161,
            80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55,
            160, 199, 184, 78, 250,
        ];

        let ser = minicbor::to_vec(&info).unwrap();
        assert_eq!(ser, expected);
        let de = minicbor::decode::<PieceInfo>(&ser).unwrap();
        assert_eq!(de, info);
    }

    #[test]
    fn test_piece_info_json_serde() {
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        let info = PieceInfo {
            size: PaddedPieceSize::new(128).unwrap(),
            piece_cid: cid,
        };
        let expected = "{\
            \"Size\":128,\
            \"PieceCID\":{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"}\
        }";

        let ser = serde_json::to_string(&info).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<PieceInfo>(&ser).unwrap();
        assert_eq!(de, info);
    }
}

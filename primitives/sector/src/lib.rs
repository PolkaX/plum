// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! Some structures about sector.

#![deny(missing_docs)]

mod posting;
mod sealing;
mod sector;

pub use self::posting::{PoStProof, WindowPoStVerifyInfo, WinningPoStVerifyInfo};
pub use self::sealing::SealVerifyInfo;
pub use self::sector::{
    readable_sector_size, RegisteredPoStProof, RegisteredSealProof, SectorId, SectorInfo,
    SectorNumber, SectorQuality, SectorSize, SpaceTime, StoragePower, UnknownSectorSizeErr,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readable_sector_size() {
        let kib = 1024;
        let pib = 1_125_899_906_842_624;
        assert_eq!(readable_sector_size(0), "0B");
        assert_eq!(readable_sector_size(1), "1B");
        assert_eq!(readable_sector_size(1023), "1023B");
        assert_eq!(readable_sector_size(kib), "1KiB");
        assert_eq!(readable_sector_size(kib + 1), "1KiB");
        assert_eq!(readable_sector_size(2 * kib - 1), "1KiB");
        assert_eq!(readable_sector_size(2 * kib), "2KiB");
        assert_eq!(readable_sector_size(2 * kib + 1), "2KiB");
        assert_eq!(readable_sector_size(1023 * kib), "1023KiB");
        assert_eq!(readable_sector_size(1_048_576), "1MiB");
        assert_eq!(readable_sector_size(1_073_741_824), "1GiB");
        assert_eq!(readable_sector_size(1_099_511_627_776), "1TiB");
        assert_eq!(readable_sector_size(pib), "1PiB");
        assert_eq!(readable_sector_size(kib * pib), "1EiB");
        assert_eq!(readable_sector_size(10 * kib * pib), "10EiB");
    }

    #[test]
    fn test_cbor_and_json_serde() {
        use cid::Cid;
        use minicbor::{decode, encode};
        use serde::{de::DeserializeOwned, Serialize};

        trait DecodeOwned: for<'b> decode::Decode<'b> {}
        impl<T> DecodeOwned for T where T: for<'b> decode::Decode<'b> {}

        fn asset_cbor<T>(obj: &T, expect: Vec<u8>)
        where
            T: DecodeOwned + encode::Encode + PartialEq + std::fmt::Debug,
        {
            let ser = minicbor::to_vec(obj).unwrap();
            assert_eq!(ser, expect);
            let de = minicbor::decode::<T>(&ser).unwrap();
            assert_eq!(&de, obj);
        }

        fn assert_json<T>(obj: &T, expect: &str)
        where
            T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
        {
            let ser = serde_json::to_string(&obj).unwrap();
            assert_eq!(ser, expect);
            let de = serde_json::from_str::<T>(&ser).unwrap();
            assert_eq!(&de, obj);
        }

        // SectorId
        let sector_id = SectorId {
            miner: 100,
            number: 100,
        };
        asset_cbor(&sector_id, vec![130, 24, 100, 24, 100]);
        assert_json(&sector_id, "{\"Miner\":100,\"Number\":100}");

        // RegisteredSealProof
        let seal_proof = RegisteredSealProof::StackedDrg512MiBV1;
        asset_cbor(&seal_proof, vec![2]);
        assert_json(&seal_proof, "2");

        // SectorInfo
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();
        let sector_info = SectorInfo {
            seal_proof,
            sector_number: 1111,
            sealed_cid: cid.clone(),
        };
        asset_cbor(
            &sector_info,
            vec![
                131, 2, 25, 4, 87, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29,
                97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225,
                217, 73, 55, 160, 199, 184, 78, 250,
            ],
        );
        assert_json(
            &sector_info,
            "{\
                \"SealProof\":2,\
                \"SectorNumber\":1111,\
                \"SealedCID\":{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"}\
            }",
        );

        // SealVerifyInfo
        let _info = SealVerifyInfo {
            seal_proof,
            sector_id,
            randomness: vec![1; 32].into(),
            interactive_randomness: vec![2; 32].into(),
            proof: vec![1, 2, 3, 4, 5, 6, 7, 8],
            sealed_cid: cid.clone(),
            unsealed_cid: cid,
            deal_ids: vec![8, 7, 6],
        };
        // TODO need check

        // RegisteredSealProof
        let post_proof = RegisteredPoStProof::StackedDrgWinning64GiBV1;
        asset_cbor(&post_proof, vec![4]);
        assert_json(&post_proof, "4");
        // PoStProof
        let post_proof = PoStProof {
            post_proof,
            proof_bytes: vec![1, 2, 3],
        };
        asset_cbor(&post_proof, vec![130, 4, 67, 1, 2, 3]);
        assert_json(&post_proof, "{\"PoStProof\":4,\"ProofBytes\":\"AQID\"}");

        // WinningPoStVerifyInfo and WindowPoStVerifyInfo
        let post_verify_info = WinningPoStVerifyInfo {
            randomness: vec![1; 32].into(),
            proofs: vec![post_proof],
            challenged_sectors: vec![sector_info],
            prover: 0,
        };
        asset_cbor(
            &post_verify_info,
            vec![
                132, 88, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 129, 130, 4, 67, 1, 2, 3, 129, 131, 2, 25, 4, 87, 216,
                42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49,
                47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199, 184,
                78, 250, 0,
            ],
        );
        assert_json(
            &post_verify_info,
            "{\
                \"Randomness\":\"AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=\",\
                \"Proofs\":[\
                    {\"PoStProof\":4,\"ProofBytes\":\"AQID\"}\
                ],\
                \"ChallengedSectors\":[\
                    {\
                        \"SealProof\":2,\
                        \"SectorNumber\":1111,\
                        \"SealedCID\":{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"}\
                    }\
                ],\
                \"Prover\":0\
                }",
        );
    }
}

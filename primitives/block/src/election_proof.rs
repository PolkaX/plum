// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

/// The PoSt election proof of space/time
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct ElectionProof {
    /// VRF proof
    pub vrf_proof: Vec<u8>,
}

impl ser::Serialize for ElectionProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for ElectionProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// ElectionProof CBOR serialization/deserialization
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};
    use serde_bytes::{ByteBuf, Bytes};

    use super::ElectionProof;

    /// CBOR serialization
    pub fn serialize<S>(election_proof: &ElectionProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = Bytes::new(&election_proof.vrf_proof);
        (value,).serialize(serializer)
    }

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<ElectionProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (vrf_proof,) = <(ByteBuf,)>::deserialize(deserializer)?;
        Ok(ElectionProof {
            vrf_proof: vrf_proof.into_vec(),
        })
    }

    #[test]
    fn election_proof_cbor_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CborElectionProof(#[serde(with = "self")] ElectionProof);

        let cases = vec![(
            CborElectionProof(ElectionProof {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            }),
            vec![
                129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48,
                48, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
            ],
        )];

        for (election_proof, expected) in cases {
            let ser = serde_cbor::to_vec(&election_proof).unwrap();
            assert_eq!(ser, &expected[..]);
            let de = serde_cbor::from_slice::<CborElectionProof>(&ser).unwrap();
            assert_eq!(de, election_proof);
        }
    }
}

/// ElectionProof JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::ElectionProof;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonElectionProofRef<'a> {
        #[serde(rename = "VRFProof")]
        #[serde(with = "plum_types::base64")]
        vrf_proof: &'a [u8],
    }

    /// JSON serialization
    pub fn serialize<S>(election_proof: &ElectionProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonElectionProofRef {
            vrf_proof: &election_proof.vrf_proof,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonElectionProof {
        #[serde(rename = "VRFProof")]
        #[serde(with = "plum_types::base64")]
        vrf_proof: Vec<u8>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<ElectionProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonElectionProof { vrf_proof } = JsonElectionProof::deserialize(deserializer)?;
        Ok(ElectionProof { vrf_proof })
    }

    #[test]
    fn election_proof_json_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct JsonElectionProof(#[serde(with = "self")] ElectionProof);

        let cases = vec![(
            JsonElectionProof(ElectionProof {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            }),
            r#"{"VRFProof":"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA="}"#,
        )];

        for (election_proof, expected) in cases {
            let ser = serde_json::to_string(&election_proof).unwrap();
            assert_eq!(ser, expected);
            let de = serde_json::from_str::<JsonElectionProof>(&ser).unwrap();
            assert_eq!(de, election_proof);
        }
    }
}

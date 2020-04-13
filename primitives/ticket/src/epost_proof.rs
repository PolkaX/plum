// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use crate::epost_ticket::EPostTicket;

/// The PoSt election proof of space/time
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct EPostProof {
    ///
    pub proof: Vec<u8>,
    ///
    pub post_rand: Vec<u8>,
    ///
    pub candidates: Vec<EPostTicket>,
}

impl ser::Serialize for EPostProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for EPostProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// EPostProof CBOR serialization/deserialization
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use super::{EPostProof, EPostTicket};

    #[derive(Serialize)]
    struct CborEPostProofRef<'a>(
        #[serde(with = "serde_bytes")] &'a [u8],
        #[serde(with = "serde_bytes")] &'a [u8],
        #[serde(with = "crate::epost_ticket::cbor::vec")] &'a [EPostTicket],
    );

    /// CBOR serialization
    pub fn serialize<S>(epost_proof: &EPostProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborEPostProofRef(
            &epost_proof.proof,
            &epost_proof.post_rand,
            &epost_proof.candidates,
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborEPostProof(
        #[serde(with = "serde_bytes")] Vec<u8>,
        #[serde(with = "serde_bytes")] Vec<u8>,
        #[serde(with = "crate::epost_ticket::cbor::vec")] Vec<EPostTicket>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<EPostProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborEPostProof(proof, post_rand, candidates) =
            CborEPostProof::deserialize(deserializer)?;
        Ok(EPostProof {
            proof,
            post_rand,
            candidates,
        })
    }

    #[test]
    fn epost_proof_cbor_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CborEPostProof(#[serde(with = "self")] EPostProof);

        let cases = vec![(
            CborEPostProof(EPostProof {
                proof: b"pruuf".to_vec(),
                post_rand: b"random".to_vec(),
                candidates: vec![],
            }),
            vec![
                131, 69, 112, 114, 117, 117, 102, 70, 114, 97, 110, 100, 111, 109, 128,
            ],
        )];

        for (epost_proof, expected) in cases {
            let ser = serde_cbor::to_vec(&epost_proof).unwrap();
            assert_eq!(ser, &expected[..]);
            let de = serde_cbor::from_slice::<CborEPostProof>(&ser).unwrap();
            assert_eq!(de, epost_proof);
        }
    }
}

/// EPostProof JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::{EPostProof, EPostTicket};

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonEPostProofRef<'a> {
        #[serde(with = "plum_types::base64")]
        proof: &'a [u8],
        #[serde(with = "plum_types::base64")]
        post_rand: &'a [u8],
        #[serde(with = "crate::epost_ticket::json::vec")]
        candidates: &'a [EPostTicket],
    }

    /// JSON serialization
    pub fn serialize<S>(epost_proof: &EPostProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonEPostProofRef {
            proof: &epost_proof.proof,
            post_rand: &epost_proof.post_rand,
            candidates: &epost_proof.candidates,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonEPostProof {
        #[serde(with = "plum_types::base64")]
        proof: Vec<u8>,
        #[serde(with = "plum_types::base64")]
        post_rand: Vec<u8>,
        #[serde(with = "crate::epost_ticket::json::vec")]
        candidates: Vec<EPostTicket>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<EPostProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonEPostProof {
            proof,
            post_rand,
            candidates,
        } = JsonEPostProof::deserialize(deserializer)?;
        Ok(EPostProof {
            proof,
            post_rand,
            candidates,
        })
    }

    #[test]
    fn epost_proof_json_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct JsonEPostProof(#[serde(with = "self")] EPostProof);

        let cases = vec![(
            JsonEPostProof(EPostProof {
                proof: b"pruuf".to_vec(),
                post_rand: b"random".to_vec(),
                candidates: vec![],
            }),
            r#"{"Proof":"cHJ1dWY=","PostRand":"cmFuZG9t","Candidates":[]}"#,
        )];

        for (epost_proof, expected) in cases {
            let ser = serde_json::to_string(&epost_proof).unwrap();
            assert_eq!(ser, expected);
            let de = serde_json::from_str::<JsonEPostProof>(&ser).unwrap();
            assert_eq!(de, epost_proof);
        }
    }
}

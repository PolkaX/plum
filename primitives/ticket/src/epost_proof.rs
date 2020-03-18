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
    struct CborEPostTicketRef<'a>(#[serde(with = "crate::epost_ticket::cbor")] &'a EPostTicket);

    #[derive(Serialize)]
    struct TupleEPostProofRef<'a>(
        #[serde(with = "serde_bytes")] &'a [u8],
        #[serde(with = "serde_bytes")] &'a [u8],
        &'a [CborEPostTicketRef<'a>],
    );

    /// CBOR serialization
    pub fn serialize<S>(epost_proof: &EPostProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        TupleEPostProofRef(
            &epost_proof.proof,
            &epost_proof.post_rand,
            &epost_proof
                .candidates
                .iter()
                .map(|candidates| CborEPostTicketRef(candidates))
                .collect::<Vec<_>>(),
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborEPostTicket(#[serde(with = "crate::epost_ticket::cbor")] EPostTicket);

    #[derive(Deserialize)]
    struct TupleEPostProof(
        #[serde(with = "serde_bytes")] Vec<u8>,
        #[serde(with = "serde_bytes")] Vec<u8>,
        Vec<CborEPostTicket>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<EPostProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleEPostProof(proof, post_rand, candidates) =
            TupleEPostProof::deserialize(deserializer)?;
        let candidates = candidates
            .into_iter()
            .map(|candidate| candidate.0)
            .collect();
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
    struct JsonEPostTicketRef<'a>(#[serde(with = "crate::epost_ticket::json")] &'a EPostTicket);

    #[derive(Serialize)]
    struct JsonEPostProofRef<'a> {
        #[serde(rename = "Proof")]
        proof: String,
        #[serde(rename = "PostRand")]
        post_rand: String,
        #[serde(rename = "Candidates")]
        candidates: &'a [JsonEPostTicketRef<'a>],
    }

    /// CBOR serialization
    pub fn serialize<S>(epost_proof: &EPostProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonEPostProofRef {
            proof: base64::encode(&epost_proof.proof),
            post_rand: base64::encode(&epost_proof.post_rand),
            candidates: &epost_proof
                .candidates
                .iter()
                .map(|candidate| JsonEPostTicketRef(candidate))
                .collect::<Vec<_>>(),
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonEPostProof {
        #[serde(rename = "Proof")]
        proof: String,
        #[serde(rename = "PostRand")]
        post_rand: String,
        #[serde(rename = "Candidates")]
        candidates: Vec<EPostTicket>,
    }

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<EPostProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let epost_proof = JsonEPostProof::deserialize(deserializer)?;
        Ok(EPostProof {
            proof: base64::decode(epost_proof.proof).expect("base64 decode shouldn't be fail"),
            post_rand: base64::decode(epost_proof.post_rand)
                .expect("base64 decode shouldn't be fail"),
            candidates: epost_proof.candidates,
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
            vec![
                123, 34, 80, 114, 111, 111, 102, 34, 58, 34, 99, 72, 74, 49, 100, 87, 89, 61, 34,
                44, 34, 80, 111, 115, 116, 82, 97, 110, 100, 34, 58, 34, 99, 109, 70, 117, 90, 71,
                57, 116, 34, 44, 34, 67, 97, 110, 100, 105, 100, 97, 116, 101, 115, 34, 58, 91, 93,
                125,
            ],
            r#"{"Proof":"cHJ1dWY=","PostRand":"cmFuZG9t","Candidates":[]}"#,
        )];

        for (epost_proof, expected_bytes, expected_str) in cases {
            let ser = serde_json::to_vec(&epost_proof).unwrap();
            assert_eq!(ser, expected_bytes);
            let de = serde_json::from_slice::<JsonEPostProof>(&ser).unwrap();
            assert_eq!(de, epost_proof);

            let ser = serde_json::to_string(&epost_proof).unwrap();
            assert_eq!(ser, expected_str);
            let de = serde_json::from_str::<JsonEPostProof>(&ser).unwrap();
            assert_eq!(de, epost_proof);
        }
    }
}

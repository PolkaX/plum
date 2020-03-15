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
        let candidates = epost_proof
            .candidates
            .iter()
            .map(|candidates| CborEPostTicketRef(candidates))
            .collect::<Vec<_>>();
        TupleEPostProofRef(&epost_proof.proof, &epost_proof.post_rand, &candidates)
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
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::EPostProof;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CborEPostProof(#[serde(with = "super::cbor")] EPostProof);

    #[test]
    fn epost_proof_cbor_serde() {
        let epost_proof = CborEPostProof(EPostProof {
            proof: b"pruuf".to_vec(),
            post_rand: b"random".to_vec(),
            candidates: Vec::new(),
        });
        let expected = [
            131, 69, 112, 114, 117, 117, 102, 70, 114, 97, 110, 100, 111, 109, 128,
        ];

        let ser = serde_cbor::to_vec(&epost_proof).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice::<CborEPostProof>(&ser).unwrap();
        assert_eq!(de, epost_proof);
    }
}

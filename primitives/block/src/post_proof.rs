// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde_repr::{Deserialize_repr, Serialize_repr};

///
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
pub enum RegisteredProof {
    StackedDRG32GiBSeal = 1,
    StackedDRG32GiBPoSt = 2, // No longer used
    StackedDRG2KiBSeal = 3,  // No longer used
    StackedDRG2KiBPoSt = 4,
    StackedDRG8MiBSeal = 5, // No longer used
    StackedDRG8MiBPoSt = 6,
    StackedDRG512MiBSeal = 7, // No longer used
    StackedDRG512MiBPoSt = 8,

    StackedDRG2KiBWinningPoSt = 9,
    StackedDRG2KiBWindowPoSt = 10,
    StackedDRG8MiBWinningPoSt = 11,
    StackedDRG8MiBWindowPoSt = 12,
    StackedDRG512MiBWinningPoSt = 13,
    StackedDRG512MiBWindowPoSt = 14,
    StackedDRG32GiBWinningPoSt = 15,
    StackedDRG32GiBWindowPoSt = 16,
}

///
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PoStProof {
    pub registered_proof: RegisteredProof,
    pub proof_bytes: Vec<u8>,
}

/// PoStProof CBOR serialization/deserialization
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use super::{PoStProof, RegisteredProof};

    #[derive(Serialize)]
    struct CborPoStProofRef<'a>(
        &'a RegisteredProof,
        #[serde(with = "serde_bytes")] &'a Vec<u8>,
    );

    /// CBOR serialization
    pub fn serialize<S>(post_proof: &PoStProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborPoStProofRef(&post_proof.registered_proof, &post_proof.proof_bytes)
            .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborPoStProof(RegisteredProof, #[serde(with = "serde_bytes")] Vec<u8>);

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<PoStProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborPoStProof(registered_proof, proof_bytes) =
            CborPoStProof::deserialize(deserializer)?;
        Ok(PoStProof {
            registered_proof,
            proof_bytes,
        })
    }

    /// Vec<PoStProof> CBOR serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct CborPoStProofRef<'a>(#[serde(with = "super")] &'a PoStProof);

        /// CBOR serialization of Vec<PoStProof>.
        pub fn serialize<S>(post_proofs: &[PoStProof], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            post_proofs
                .iter()
                .map(|post_proof| CborPoStProofRef(post_proof))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct CborPoStProof(#[serde(with = "super")] PoStProof);

        /// CBOR deserialization of Vec<PoStProof>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<PoStProof>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let post_proofs = <Vec<CborPoStProof>>::deserialize(deserializer)?;
            Ok(post_proofs
                .into_iter()
                .map(|CborPoStProof(post_proof)| post_proof)
                .collect())
        }
    }
}

/// PoStProof JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::{PoStProof, RegisteredProof};

    #[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct JsonPoStProofRef<'a> {
        registered_proof: &'a RegisteredProof,
        #[serde(with = "plum_types::base64")]
        proof_bytes: &'a [u8],
    }

    /// JSON serialization
    pub fn serialize<S>(post_proof: &PoStProof, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonPoStProofRef {
            registered_proof: &post_proof.registered_proof,
            proof_bytes: &post_proof.proof_bytes,
        }
        .serialize(serializer)
    }

    #[derive(Eq, PartialEq, Debug, Clone, Hash, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct JsonPoStProof {
        registered_proof: RegisteredProof,
        #[serde(with = "plum_types::base64")]
        proof_bytes: Vec<u8>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<PoStProof, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonPoStProof {
            registered_proof,
            proof_bytes,
        } = JsonPoStProof::deserialize(deserializer)?;
        Ok(PoStProof {
            registered_proof,
            proof_bytes,
        })
    }

    /// Vec<BeaconEntry> JSON serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct JsonPoStProofRef<'a>(#[serde(with = "super")] &'a PoStProof);

        /// JSON serialization of Vec<BeaconEntry>.
        pub fn serialize<S>(post_proofs: &[PoStProof], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            post_proofs
                .iter()
                .map(|post_proof| JsonPoStProofRef(post_proof))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct JsonPoStProof(#[serde(with = "super")] PoStProof);

        /// JSON deserialization of Vec<BeaconEntry>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<PoStProof>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let post_proofs = <Vec<JsonPoStProof>>::deserialize(deserializer)?;
            Ok(post_proofs
                .into_iter()
                .map(|JsonPoStProof(post_proof)| post_proof)
                .collect())
        }
    }
}

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use super::sector::PrivatePoStCandidateProof;

pub mod option_prev_post_candidate_proof {
    use super::*;
    use crate::abi::sector::RegisteredProof;
    use std::convert::TryFrom;

    #[derive(Serialize, Deserialize)]
    struct TmpProof(usize, #[serde(with = "serde_bytes")] Vec<u8>);
    pub fn serialize<S>(
        opt: &Option<PrivatePoStCandidateProof>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(proof) = opt {
            (*proof).serialize(serializer)
        } else {
            TmpProof(0, vec![]).serialize(serializer)
        }
    }
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<PrivatePoStCandidateProof>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let proof = TmpProof::deserialize(deserializer)?;
        let p = match proof.0 {
            0 => None,
            _ => Some(PrivatePoStCandidateProof {
                proof: RegisteredProof::try_from(proof.0)
                    .map_err(|e| D::Error::custom(format!("{:?}", e)))?,
                externalized: proof.1,
            }),
        };
        Ok(p)
    }
}

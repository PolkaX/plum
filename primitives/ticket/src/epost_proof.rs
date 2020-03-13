// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::epost_ticket::EPostTicket;

/// The PoSt election proof of space/time
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct EPostProof {
    ///
    #[serde(with = "serde_bytes")]
    pub proof: Vec<u8>,
    ///
    #[serde(with = "serde_bytes")]
    pub post_rand: Vec<u8>,
    ///
    pub candidates: Vec<EPostTicket>,
}

#[cfg(test)]
mod tests {
    use super::EPostProof;

    #[test]
    fn epost_proof_cbor_serde() {
        let epost_proof = EPostProof {
            proof: b"pruuf".to_vec(),
            post_rand: b"random".to_vec(),
            candidates: Vec::new(),
        };
        let expected = [
            131, 69, 112, 114, 117, 117, 102, 70, 114, 97, 110, 100, 111, 109, 128,
        ];

        let ser = serde_cbor::to_vec(&epost_proof).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice::<EPostProof>(&ser).unwrap();
        assert_eq!(de, epost_proof);
    }
}

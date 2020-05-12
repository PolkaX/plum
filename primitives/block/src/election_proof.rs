// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

/// The PoSt election proof of space/time
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct ElectionProof {
    /// VRF proof
    #[serde(rename = "VRFProof")]
    #[serde(with = "plum_bytes")]
    pub vrf_proof: Vec<u8>,
}

// Implement CBOR serialization for ElectionProof.
impl encode::Encode for ElectionProof {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(1)?.bytes(&self.vrf_proof)?.ok()
    }
}

// Implement CBOR deserialization for ElectionProof.
impl<'b> decode::Decode<'b> for ElectionProof {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(1));
        Ok(ElectionProof {
            vrf_proof: d.bytes()?.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ElectionProof;

    #[test]
    fn election_proof_cbor_serde() {
        let cases = vec![(
            ElectionProof {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            },
            vec![
                129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48,
                48, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
            ],
        )];

        for (election_proof, expected) in cases {
            let ser = minicbor::to_vec(&election_proof).unwrap();
            assert_eq!(ser, &expected[..]);
            let de = minicbor::decode::<ElectionProof>(&ser).unwrap();
            assert_eq!(de, election_proof);
        }
    }

    #[test]
    fn election_proof_json_serde() {
        let cases = vec![(
            ElectionProof {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            },
            r#"{"VRFProof":"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA="}"#,
        )];

        for (election_proof, expected) in cases {
            let ser = serde_json::to_string(&election_proof).unwrap();
            assert_eq!(ser, expected);
            let de = serde_json::from_str::<ElectionProof>(&ser).unwrap();
            assert_eq!(de, election_proof);
        }
    }
}

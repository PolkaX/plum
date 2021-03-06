// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

/// The result from getting an entry from Drand.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BeaconEntry {
    round: u64,
    #[serde(with = "plum_bytes::base64")]
    data: Vec<u8>,
}

impl BeaconEntry {
    /// Create a new BeachEntry with given round and data.
    pub fn new(round: u64, data: Vec<u8>) -> Self {
        Self { round, data }
    }

    /// Return the current round number.
    pub fn round(&self) -> u64 {
        self.round
    }

    /// Return the signature of public rand response.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

// Implement CBOR serialization for BeaconEntry.
impl encode::Encode for BeaconEntry {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?.u64(self.round)?.bytes(&self.data)?.ok()
    }
}

// Implement CBOR deserialization for BeaconEntry.
impl<'b> decode::Decode<'b> for BeaconEntry {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(BeaconEntry {
            round: d.u64()?,
            data: d.bytes()?.to_vec(),
        })
    }
}

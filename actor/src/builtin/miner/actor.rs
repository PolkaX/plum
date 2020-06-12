// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};

use plum_sector::SectorNumber;

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProveCommitSectorParams {
    pub sector_number: SectorNumber,
    pub proof: Vec<u8>,
}

impl minicbor::Encode for ProveCommitSectorParams {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?
            .u64(self.sector_number)?
            .bytes(&self.proof)?
            .ok()
    }
}

impl<'b> decode::Decode<'b> for ProveCommitSectorParams {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(ProveCommitSectorParams {
            sector_number: d.u64()?,
            proof: d.bytes()?.to_vec(),
        })
    }
}

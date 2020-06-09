// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use plum_crypto::DomainSeparationTag;
use plum_hashing::blake2b_256;
use plum_types::ChainEpoch;
use std::io::Write;

/// Computes a pseudorandom 32 byte Vec
pub fn draw_randomness(
    rbase: &[u8],
    pers: DomainSeparationTag,
    round: ChainEpoch,
    entropy: &[u8],
) -> Result<[u8; 32]> {
    let mut data = Vec::new();
    data.write_i64::<BigEndian>(pers as i64)?;
    let vrf_digest = blake2b_256(rbase);
    data.write_all(&vrf_digest)?;
    data.write_i64::<BigEndian>(round as i64)?;
    data.write_all(entropy)?;
    Ok(blake2b_256(data))
}

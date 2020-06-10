// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod comm;

pub use self::comm::{
    cid_to_data_commitment_v1, cid_to_piece_commitment_v1, cid_to_replica_commitment_v1,
    commitment_to_cid, data_commitment_v1_to_cid, piece_commitment_v1_to_cid,
    replica_commitment_v1_to_cid, CommCidErr, FILECOIN_CODEC_TYPE,
};

use plum_address::{Address, AddressError};
use plum_types::ActorId;

/// Convert actorid to prove id
pub fn to_prove_id(actor_id: ActorId) -> Result<[u8; 32], AddressError> {
    let addr = Address::new_id_addr(actor_id)?;
    let mut res: [u8; 32] = Default::default();
    let payload = addr.payload();
    let len = std::cmp::min(payload.len(), res.len());
    res[..len].copy_from_slice(&payload[..len]);
    Ok(res)
}

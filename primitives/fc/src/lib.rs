// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod commcid;

pub use self::commcid::*;

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

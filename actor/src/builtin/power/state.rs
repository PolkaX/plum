// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use plum_bigint::bigint_json;
use plum_sector::StoragePower;

///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Claim {
    /// Sum of raw byte power for a miner's sectors.
    #[serde(with = "bigint_json")]
    pub raw_byte_power: StoragePower,
    /// Sum of quality adjusted power for a miner's sectors.
    #[serde(with = "bigint_json")]
    pub quality_adj_power: StoragePower,
}

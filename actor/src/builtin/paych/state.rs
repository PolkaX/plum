// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

/// Specifies which `lane`s to be merged with what `nonce` on channelUpdate
#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Merge {
    pub lane: u64,
    pub nonce: u64,
}

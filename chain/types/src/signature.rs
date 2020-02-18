// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::key_info::KeyType;
use serde::{Deserialize, Serialize};

/// The maximum length of signature.
pub const SIGNATURE_MAX_LENGTH: u32 = 200;

/// The general signature structure.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// The key type.
    pub ty: KeyType,
    /// Tha actual signature data.
    pub data: Vec<u8>,
}

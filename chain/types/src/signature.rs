// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::key_info::KeyType;

/// The maximum length of signature.
pub const SIGNATURE_MAX_LENGTH: u32 = 200;

/// The general signature structure.
#[derive(PartialEq, Eq, Clone)]
pub struct Signature {
    /// The key type.
    pub ty: KeyType,
    /// Tha actual signature data.
    pub data: Vec<u8>,
}

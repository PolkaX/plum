// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::BigInt;

/// Number of token units in an abstract "FIL" token.
/// The network works purely in the indivisible token amounts. This constant converts to a fixed decimal with more
/// human-friendly scale.
pub const TOKEN_PRECISION: u64 = 1_000_000_000_000_000_000;

lazy_static::lazy_static! {
    /// The maximum supply of Filecoin that will ever exist (in token units)
    pub static ref TOTAL_FILECOIN: BigInt = BigInt::from(2_000_000_000) * TOKEN_PRECISION;
}

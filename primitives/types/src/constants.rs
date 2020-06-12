// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::BigInt;
use lazy_static::lazy_static;

/// Number of token units in an abstract "FIL" token.
/// The network works purely in the indivisible token amounts. This constant converts to a fixed decimal with more
/// human-friendly scale.
pub const TOKEN_PRECISION: u64 = 1_000_000_000_000_000_000;

lazy_static! {
    /// The maximum supply of Filecoin that will ever exist (in token units)
    pub static ref TOTAL_FILECOIN: BigInt = BigInt::from(2_000_000_000) * TOKEN_PRECISION;
    /// The maximum supply of Filecoin that will ever exist (in token units)
    pub static ref TOTAL_MINING_REWARD: BigInt = BigInt::from(1_400_000_000) * TOKEN_PRECISION;
}

/// TODO: move to actors
pub const TICKET_RANDOMNESS_LOOKBACK: u64 = 1;

///
pub const BLOCK_DELAY: u64 = 45;
///
pub const PROPAGATION_DELAY: u64 = 5;

///////////////
// Limits
///////////////
/// TODO: If this is gonna stay, it should move to specs-actors
/// Maximum number of messages to be included in a Block.
pub const BLOCK_MESSAGE_LIMIT: u64 = 512;
///
pub const BLOCK_GAS_LIMIT: u64 = 100_000_000;

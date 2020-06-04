// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::builtin::network::{EPOCH_DURATION_SECONDS, EPOCH_IN_DAY};

/// The period over which all a miner's active sectors will be challenged.
pub const W_POST_PROVING_PERIOD: u64 = EPOCH_IN_DAY; // 24 hours
/// The duration of a deadline's challenge window, the period before a deadline when the challenge is available.
pub const W_POST_CHALLENGE_WINDOW: u64 = 3600 / EPOCH_DURATION_SECONDS; // An hour (=24 per day)
/// The number of non-overlapping PoSt deadlines in each proving period.
pub const W_POST_PERIOD_DEADLINES: u64 = W_POST_PROVING_PERIOD / W_POST_CHALLENGE_WINDOW;

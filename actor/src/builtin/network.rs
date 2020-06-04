// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

// The duration of a chain epoch.
// This is used for deriving epoch-denominated periods that are more naturally expressed in clock time.
// TODO: In lieu of a real configuration mechanism for this value, we'd like to make it a var so that implementations
// can override it at runtime. Doing so requires changing all the static references to it in this repo to go through
// late-binding function calls, or they'll see the "wrong" value.
// https://github.com/filecoin-project/specs-actors/issues/353

const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_DAY: u64 = 86400;
const SECONDS_IN_YEAR: u64 = 31556925;
/// Represents how many seconds in an epoch.
pub const EPOCH_DURATION_SECONDS: u64 = 25;
/// Represents how many epochs in an hour.
pub const EPOCH_IN_HOUR: u64 = SECONDS_IN_HOUR / EPOCH_DURATION_SECONDS;
/// Represents how many epochs in an day.
pub const EPOCH_IN_DAY: u64 = SECONDS_IN_DAY / EPOCH_DURATION_SECONDS;
/// Represents how many epochs in an year.
pub const EPOCH_IN_YEAR: u64 = SECONDS_IN_YEAR / EPOCH_DURATION_SECONDS;

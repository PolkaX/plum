// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Specifies a domain for randomness generation.
#[doc(hidden)]
#[repr(u8)]
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum DomainSeparationTag {
    TicketProduction = 1,
    ElectionProofProduction,
    WinningPoStChallengeSeed,
    WindowedPoStChallengeSeed,
    SealRandomness,
    InteractiveSealChallengeSeed,
    WindowedPoStDeadlineAssignment,
}

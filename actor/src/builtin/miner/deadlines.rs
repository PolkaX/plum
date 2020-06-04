// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use plum_types::ChainEpoch;

/// Deadline calculations with respect to a current epoch.
/// "Deadline" refers to the window during which proofs may be submitted.
/// Windows are non-overlapping ranges [open, close), but the challenge epoch for a window occurs
/// before the window opens.
/// The current epoch may not necessarily lie within the deadline or proving period represented here.
#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeadlineInfo {
    pub current_epoch: ChainEpoch, // Epoch at which this info was calculated.
    pub period_start: ChainEpoch,  // First epoch of the proving period (<= current_epoch).
    pub index: u64, // A deadline index, in [0..WPoStProvingPeriodDeadlines) unless period elapsed.
    pub open: ChainEpoch, // First epoch from which a proof may be submitted, inclusive (>= current_epoch).
    pub close: ChainEpoch, // First epoch from which a proof may no longer be submitted, exclusive (>= open).
    pub challenge: ChainEpoch, // Epoch at which to sample the chain for challenge (< open).
    pub fault_cutoff: ChainEpoch, // First epoch at which a fault declaration is rejected (< open).
}

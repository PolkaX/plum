// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TipsetError {
    #[error("zero length array of blocks")]
    EmptyBlocks,
    #[error("mismatching heights (expected {expected:?}, found {found:?})")]
    MismatchingHeight { expected: u64, found: u64 },
    #[error("mismatching parents (expected {expected:?}, found {found:?})")]
    MismatchingParent { expected: Cid, found: Cid },
}

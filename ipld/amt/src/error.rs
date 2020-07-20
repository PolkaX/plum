// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub(crate) type Result<T, E = IpldAmtError> = std::result::Result<T, E>;

/// The IPLD AMT error.
#[doc(hidden)]
#[derive(Debug, thiserror::Error)]
pub enum IpldAmtError {
    #[error("cid is not found in the store")]
    CidNotFound,
    #[error("index `{0}` is out of range for the amt")]
    IndexOutOfRange(usize),
    #[error("store error: {0}")]
    Store(#[from] std::io::Error),
}

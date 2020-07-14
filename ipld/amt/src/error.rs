// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub(crate) type Result<T, E = IpldAmtError> = std::result::Result<T, E>;

///
#[doc(hidden)]
#[derive(Debug, thiserror::Error)]
pub enum IpldAmtError {
    #[error("cid not found")]
    CidNotFound,
    #[error("store error")]
    Store(#[from] anyhow::Error),
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub(crate) type Result<T> = std::result::Result<T, IpldAmtError>;

///
#[doc(hidden)]
#[derive(Debug, thiserror::Error)]
pub enum IpldAmtError {
    #[error("not found")]
    NotFound,
}

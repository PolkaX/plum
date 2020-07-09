// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

///
#[derive(Debug, thiserror::Error)]
pub enum HamtError {
    #[error("{0}")]
    CidNotFound(String),
}

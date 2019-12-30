// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use std::io;
use thiserror;

/// util error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error.
    #[error("Invalid password")]
    Io(io::Error),
    /// Keystore unavailable
    #[error("Keystore unavailable")]
    Unavailable,
}

impl From<io::Error> for Error {
    fn from (e: io::Error) -> Self {
        Error::Io(e)
    }
}
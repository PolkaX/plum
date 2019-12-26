// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address;
use std::io;

/// Keystore error.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// JSON error.
    Json(serde_json::Error),
    Address(address::error::Error),
    /// Invalid password.
    #[display(fmt = "Invalid password")]
    InvalidPassword,
    #[display(fmt = "Invalid Length")]
    InvalidLength,
    /// Keystore unavailable
    #[display(fmt = "Keystore unavailable")]
    Unavailable,
}

impl From<std::convert::Infallible> for Error {
    fn from(inf: std::convert::Infallible) -> Self {
        match inf {
            _ => Error::Unavailable,
        }
    }
}

// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address;
use std::io;
use thiserror;

/// Keystore error.
#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum Error {
    /// IO error.
    #[error("Invalid password")]
    Io(io::Error),
    /// JSON error.
    #[error("Json error")]
    Json(serde_json::Error),
    #[error("address error")]
    Address(address::error::Error),
    /// Invalid password.
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid Length")]
    InvalidLength,
    /// Keystore unavailable
    #[error("Keystore unavailable")]
    Unavailable,
}

impl From<std::convert::Infallible> for Error {
    fn from(inf: std::convert::Infallible) -> Self {
        match inf {
            _ => Error::Unavailable,
        }
    }
}

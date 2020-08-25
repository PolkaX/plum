// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// RLE+ error.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Run length too large for RLE+ version.
    RunLengthTooLarge,
    /// RLE+ decode error.
    Decode(String),
    /// Invalid RLE+ version.
    WrongVersion,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RunLengthTooLarge => f.write_str("run length too large for RLE+ version"),
            Error::Decode(err) => f.write_str(err),
            Error::WrongVersion => f.write_str("invalid RLE+ version"),
        }
    }
}

impl std::error::Error for Error {}

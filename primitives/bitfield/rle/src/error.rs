// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

///
pub type Result<T> = std::result::Result<T, RleDecodeError>;

///
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum RleDecodeError {
    ///
    #[error("RLE+ data header has invalid version")]
    VersionMismatch,
    ///
    #[error("RLE+ incorrect structure")]
    DataIndexFailure,
    ///
    #[error("RLE+ invalid encoding")]
    UnpackOverflow,
    ///
    #[error("RLE+ object size too large")]
    MaxSizeExceed,
}

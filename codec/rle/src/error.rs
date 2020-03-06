pub type Result<T> = std::result::Result<T, RLEDecodeError>;

#[derive(thiserror::Error, Debug)]
pub enum RLEDecodeError {
    #[error("RLE+ data header has invalid version.")]
    VersionMismatch,
    #[error("RLE+ incorrect structure.")]
    DataIndexFailure,
    #[error("RLE+ invalid encoding")]
    UnpackOverflow,
    #[error("RLE+ object size too large")]
    MaxSizeExceed,
}

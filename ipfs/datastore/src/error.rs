// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub(crate) type Result<T> = std::result::Result<T, DataStoreError>;

/// The error type used for data store
#[doc(hidden)]
#[derive(Clone, Debug, thiserror::Error)]
pub enum DataStoreError {
    #[error("key '{0}' not found")]
    NotFound(String),
    #[error("{0}")]
    Custom(String),
}

impl From<std::io::Error> for DataStoreError {
    fn from(err: std::io::Error) -> Self {
        DataStoreError::Custom(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DataStoreError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DataStoreError::Custom(err.to_string())
    }
}

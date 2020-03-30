use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageMarketError {
    #[error("this proposal already do sign before")]
    AlreadySigned,
    #[error("this proposal do not have signature")]
    NotSigned,
    #[error("sign error: {0}")]
    Sign(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
    #[error("sign error: {0}")]
    Ipld(#[from] ipld_cbor::IpldCborError),
    #[error("address error: {0}")]
    Address(#[from] plum_address::AddressError),
    #[error("crypto error: {0}")]
    Crypto(#[from] plum_crypto::CryptoError),
    #[error("cbor error: {0}")]
    Cbor(#[from] serde_cbor::Error),
}
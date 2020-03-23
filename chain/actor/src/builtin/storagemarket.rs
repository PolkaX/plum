use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use thiserror::Error;

use cid::Cid;
use plum_address::Address;
use plum_bigint::BigInt;
use plum_crypto::Signature;

use ipld_cbor::IpldNode;

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct StorageDealProposal {
    // cid bytes
    pub piece_ref: Cid,
    pub piece_size: u64,

    pub client: Address,
    pub provider: Address,

    pub proposal_expiration: u64,
    pub duration: u64, // TODO: spec
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub storage_price_per_epoch: BigInt,
    #[serde(with = "plum_bigint::bigint_cbor")]
    pub storage_collateral: BigInt,
    // to not share, maybe do not need option
    pub proposer_signature: Option<Signature>,
}

impl StorageDealProposal {
    pub fn total_storage_price(&self) -> BigInt {
        self.storage_price_per_epoch.clone() * self.duration
    }

    pub fn sign<F>(&mut self, sign_func: F) -> Result<(), StorageMarketError>
    where
        F: Fn(&[u8]) -> Signature,
    {
        if self.proposer_signature.is_some() {
            return Err(StorageMarketError::AlreadySigned);
        }
        // todo why use an empty bytes?
        let sign = sign_func(&vec![]);
        self.proposer_signature = Some(sign);
        Ok(())
    }

    pub fn cid(&self) -> Result<Cid, StorageMarketError> {
        let node = IpldNode::from_object(self.clone(), multihash::Code::Sha2_256)?;
        Ok(node.as_ref().clone())
    }

    /// if worker is same as self.client, then do nothing for verify.
    pub fn verify(&self, worker: Option<Address>) -> Result<(), StorageMarketError> {
        let verify_func = || -> Result<(), StorageMarketError> {
            let mut unsigned = self.clone();
            unsigned.proposer_signature = None;
            let buf = serde_cbor::to_vec(&unsigned)?;
            if let Some(ref sign) = self.proposer_signature {
                sign.check_address_type(&self.client)?;
                let pk = self.client.pubkey()?;
                if sign.verify(pk, buf)? {
                    Ok(())
                } else {
                    Err(plum_crypto::CryptoError::VerifyFailed.into())
                }
            } else {
                Err(StorageMarketError::NotSigned)
            }
        };

        if let Some(addr) = worker {
            if self.client != addr {
                verify_func()?;
            }
        // worker is same as client, do nothing
        } else {
            // worker is none
            verify_func()?;
        }
        Ok(())
    }
}

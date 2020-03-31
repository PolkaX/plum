use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use cid::Cid;
use plum_address::Address;
use plum_bigint::BigInt;
use plum_crypto::Signature;
use plum_types::DealId;

use ipld_cbor::IpldNode;

use super::error::StorageMarketError;
use crate::abi::sector::RegisteredProof;

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
                let pk = self.client.payload();
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize_tuple, Deserialize_tuple)]
pub struct ComputeDataCommitmentParams {
    pub deal_ids: Vec<DealId>,
    pub sector_type: RegisteredProof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OnMinerSectorsTerminateParams {
    pub deal_ids: Vec<DealId>,
}

impl Serialize for OnMinerSectorsTerminateParams {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        (&self.deal_ids,).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OnMinerSectorsTerminateParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r: (Vec<DealId>,) = Deserialize::deserialize(deserializer)?;
        Ok(OnMinerSectorsTerminateParams { deal_ids: r.0 })
    }
}

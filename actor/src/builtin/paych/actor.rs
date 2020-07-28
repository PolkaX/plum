// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use plum_address::Address;
use plum_bigint::{bigint_json, BigInt};
use plum_crypto::Signature;
use plum_types::{ChainEpoch, MethodNum};

use super::state::Merge;

/// A voucher is sent by `From` to `To` off-chain in order to enable
/// `To` to redeem payments on-chain in the future
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignedVoucher {
    /// time_lock_min sets a min epoch before which the voucher cannot be redeemed
    pub time_lock_min: ChainEpoch,
    /// time_lock_max sets a max epoch beyond which the voucher cannot be redeemed
    /// time_lock_max set to 0 means no timeout
    pub time_lock_max: ChainEpoch,
    /// (optional) The secret_preimage is used by `To` to validate
    #[serde(with = "plum_bytes::base64")]
    pub secret_preimage: Vec<u8>,
    /// (optional) extra can be specified by `From` to add a verification method to the voucher
    pub extra: ModVerifyParams,
    /// Specifies which lane the Voucher merges into (will be created if does not exist)
    pub lane: u64,
    /// nonce is set by `From` to prevent redemption of stale vouchers on a lane
    pub nonce: u64,
    /// amount voucher can be redeemed for
    #[serde(with = "bigint_json")]
    pub amount: BigInt,
    /// (optional) min_settle_height can extend channel MinSettleHeight if needed
    pub min_settle_height: ChainEpoch,

    /// (optional) Set of lanes to be merged into `Lane`
    pub mergers: Vec<Merge>,

    /// Sender's signature over the voucher
    pub signature: Signature,
}

/// Modular Verification method
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModVerifyParams {
    pub actor: Address,
    pub method: MethodNum,
    #[serde(with = "plum_bytes::base64")]
    pub data: Vec<u8>,
}

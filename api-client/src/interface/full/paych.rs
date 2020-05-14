// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use cid::Cid;
use plum_address::Address;
use plum_bigint::{/*bigint_json, */ BigInt, BigIntRefWrapper};
// use plum_bytes::BytesRef;
// use plum_types::ChainEpoch;

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[doc(hidden)]
#[async_trait::async_trait]
pub trait PaychApi: RpcClient {
    async fn paych_get(
        &self,
        from: &Address,
        to: &Address,
        ensure_funds: &BigInt,
    ) -> Result<ChannelInfo> {
        self.request(
            "PaychGet",
            vec![
                helper::serialize(from),
                helper::serialize(to),
                helper::serialize(&BigIntRefWrapper::from(ensure_funds)),
            ],
        )
        .await
    }

    async fn paych_list(&self) -> Result<Vec<Address>> {
        self.request("PaychList", vec![]).await
    }

    async fn paych_status(&self, addr: &Address) -> Result<PaychStatus> {
        self.request("PaychStatus", vec![helper::serialize(addr)])
            .await
    }

    async fn paych_close(&self, addr: &Address) -> Result<Cid> {
        self.request("PaychClose", vec![helper::serialize(addr)])
            .await
    }

    async fn paych_allocate_lane(&self, addr: &Address) -> Result<u64> {
        self.request("PaychAllocateLane", vec![helper::serialize(addr)])
            .await
    }

    /*
    async fn paych_new_payment(
        &self,
        from: &Address,
        to: &Address,
        vouchers: &[VoucherSpec],
    ) -> Result<PaymentInfo> {
        self.request(
            "PaychNewPayment",
            vec![
                helper::serialize(from),
                helper::serialize(to),
                helper::serialize(vouchers),
            ],
        )
        .await
    }

    async fn paych_voucher_check_valid(
        &self,
        addr: &Address,
        sign_vouch: &SignedVoucher,
    ) -> Result<()> {
        self.request(
            "PaychVoucherCheckValid",
            vec![helper::serialize(addr), helper::serialize(sign_vouch)],
        )
        .await
    }

    async fn paych_voucher_check_spendable(
        &self,
        addr: &Address,
        sign_vouch: &SignedVoucher,
        secret: &[u8],
        proof: &[u8],
    ) -> Result<bool> {
        self.request(
            "PaychVoucherCheckSpendable",
            vec![
                helper::serialize(addr),
                helper::serialize(sign_vouch),
                helper::serialize(&BytesRef::from(secret)),
                helper::serialize(&BytesRef::from(proof)),
            ],
        )
        .await
    }

    async fn paych_voucher_create(
        &self,
        addr: &Address,
        amt: &BigInt,
        lane: u64,
    ) -> Result<SignedVoucher> {
        self.request(
            "PaychVoucherCreate",
            vec![
                helper::serialize(addr),
                helper::serialize(&BigIntRefWrapper::from(amt)),
                helper::serialize(&lane),
            ],
        )
        .await
    }

    async fn paych_voucher_add(
        &self,
        addr: &Address,
        signed_vouch: &SignedVoucher,
        proof: &[u8],
        min_delta: &BigInt,
    ) -> Result<BigInt> {
        self.request(
            "PaychVoucherAdd",
            vec![
                helper::serialize(addr),
                helper::serialize(signed_vouch),
                helper::serialize(&BytesRef::from(proof)),
                helper::serialize(&BigIntRefWrapper::from(min_delta)),
            ],
        )
        .await
    }

    async fn paych_voucher_list(&self, addr: &Address) -> Result<Vec<SignedVoucher>> {
        self.request("PayChVoucherList", vec![helper::serialize(addr)])
            .await
    }

    async fn paych_voucher_submit(
        &self,
        addr: &Address,
        signed_vouch: &SignedVoucher,
    ) -> Result<Cid> {
        self.request(
            "PaychVoucherSubmit",
            vec![helper::serialize(addr), helper::serialize(signed_vouch)],
        )
        .await
    }
    */
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelInfo {
    pub channel: Address,
    pub channel_message: Cid,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaychStatus {
    pub control_addr: Address,
    pub direction: PchDir,
}

///
#[doc(hidden)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum PchDir {
    Undef = 0,
    Inbound = 1,
    Outbound = 2,
}

/*
///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub channel: Address,
    pub channel_message: Cid,
    pub vouchers: Vec<SignedVoucher>,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoucherSpec {
    #[serde(with = "bigint_json")]
    pub amount: BigInt,
    pub time_lock_min: ChainEpoch,
    pub time_lock_max: ChainEpoch,
    pub min_settle: ChainEpoch,

    pub extra: ModVerifyParams,
}
*/

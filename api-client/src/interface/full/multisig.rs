// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use plum_address::Address;
use plum_bigint::{BigInt, BigIntRefWrapper, BigIntWrapper};
use plum_bytes::BytesRef;
use plum_tipset::TipsetKey;

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
#[async_trait::async_trait]
pub trait MultiSigApi: RpcClient {
    async fn msig_get_available_balance(&self, addr: &Address, key: &TipsetKey) -> Result<BigInt> {
        let bigint: BigIntWrapper = self
            .request(
                "MsigGetAvailableBalance",
                vec![helper::serialize(addr), helper::serialize(key)],
            )
            .await?;
        Ok(bigint.into_inner())
    }

    async fn msig_create(
        &self,
        req: i64,
        addrs: &[Address],
        val: &BigInt,
        src: &Address,
        gp: &BigInt,
    ) -> Result<Cid> {
        self.request(
            "MsigCreate",
            vec![
                helper::serialize(&req),
                helper::serialize(&addrs),
                helper::serialize(&BigIntRefWrapper::from(val)),
                helper::serialize(src),
                helper::serialize(&BigIntRefWrapper::from(gp)),
            ],
        )
        .await
    }

    async fn msig_propose(
        &self,
        msig: &Address,
        to: &Address,
        amt: &BigInt,
        src: &Address,
        method: u64,
        params: &[u8],
    ) -> Result<Cid> {
        self.request(
            "MsigPropose",
            vec![
                helper::serialize(msig),
                helper::serialize(to),
                helper::serialize(&BigIntRefWrapper::from(amt)),
                helper::serialize(src),
                helper::serialize(&method),
                helper::serialize(&BytesRef::from(params)),
            ],
        )
        .await
    }

    async fn msig_approve(
        &self,
        msig: &Address,
        txid: u64,
        proposer: &Address,
        to: &Address,
        amt: &BigInt,
        src: &Address,
        method: u64,
        params: &[u8],
    ) -> Result<Cid> {
        self.request(
            "MsigApprove",
            vec![
                helper::serialize(msig),
                helper::serialize(&txid),
                helper::serialize(proposer),
                helper::serialize(to),
                helper::serialize(&BigIntRefWrapper::from(amt)),
                helper::serialize(src),
                helper::serialize(&method),
                helper::serialize(&BytesRef::from(params)),
            ],
        )
        .await
    }

    async fn msig_cancel(
        &self,
        msig: &Address,
        txid: u64,
        proposer: &Address,
        to: &Address,
        amt: &BigInt,
        src: &Address,
        method: u64,
        params: &[u8],
    ) -> Result<Cid> {
        self.request(
            "MsigCancel",
            vec![
                helper::serialize(msig),
                helper::serialize(&txid),
                helper::serialize(proposer),
                helper::serialize(to),
                helper::serialize(&BigIntRefWrapper::from(amt)),
                helper::serialize(src),
                helper::serialize(&method),
                helper::serialize(&BytesRef::from(params)),
            ],
        )
        .await
    }
}

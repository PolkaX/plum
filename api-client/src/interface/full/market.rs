// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use plum_address::Address;
use plum_bigint::{BigInt, BigIntRefWrapper};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

/// MethodGroup: Market.
#[doc(hidden)]
#[async_trait::async_trait]
pub trait MarketApi: RpcClient {
    async fn market_ensure_available(
        &self,
        addr: &Address,
        wallet: &Address,
        amt: &BigInt,
    ) -> Result<Cid> {
        self.request(
            "MarketEnsureAvailable",
            vec![
                helper::serialize(addr),
                helper::serialize(wallet),
                helper::serialize(&BigIntRefWrapper::from(amt)),
            ],
        )
        .await
    }
}

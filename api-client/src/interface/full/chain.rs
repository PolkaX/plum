// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use jsonrpc_client::{NotificationStream, SubscriptionId};

use cid::Cid;
use plum_bigint::{BigInt, BigIntWrapper};
use plum_block::BlockHeader;
use plum_bytes::{Bytes, BytesRef};
use plum_crypto::DomainSeparationTag;
use plum_message::{MessageReceipt, SignedMessage, UnsignedMessage};
use plum_tipset::{Tipset, TipsetKey};
use plum_types::{ChainEpoch, Randomness};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

/// MethodGroup: Chain.
/// The Chain method group contains methods for interacting with the blockchain,
/// but that do not require any form of state computation.
#[doc(hidden)]
#[async_trait::async_trait]
pub trait ChainApi: RpcClient {
    // ChainNotify returns channel with chain head updates
    // First message is guaranteed to be of len == 1, and type == 'current'
    async fn chain_notify(&self) -> Result<(SubscriptionId, NotificationStream<Vec<HeadChange>>)> {
        self.subscribe("ChainNotify", vec![]).await
    }

    async fn chain_head(&self) -> Result<Tipset> {
        self.request("ChainHead", vec![]).await
    }

    async fn chain_get_randomness(
        &self,
        key: &TipsetKey,
        personalization: &DomainSeparationTag,
        rand_epoch: ChainEpoch,
        entropy: &[u8],
    ) -> Result<Randomness> {
        self.request(
            "ChainGetRandomness",
            vec![
                helper::serialize(key),
                helper::serialize(personalization),
                helper::serialize(&rand_epoch),
                helper::serialize(&BytesRef::from(entropy)),
            ],
        )
        .await
    }

    async fn chain_get_block(&self, cid: &Cid) -> Result<BlockHeader> {
        self.request("ChainGetBlock", vec![helper::serialize(cid)])
            .await
    }

    async fn chain_get_tipset(&self, key: &TipsetKey) -> Result<Tipset> {
        self.request("ChainGetTipSet", vec![helper::serialize(key)])
            .await
    }

    async fn chain_get_block_messages(&self, cid: &Cid) -> Result<BlockMessages> {
        self.request("ChainGetBlockMessages", vec![helper::serialize(cid)])
            .await
    }

    async fn chain_get_parent_receipts(&self, cid: &Cid) -> Result<Vec<MessageReceipt>> {
        self.request("ChainGetParentReceipts", vec![helper::serialize(cid)])
            .await
    }

    async fn chain_get_parent_messages(&self, cid: &Cid) -> Result<Vec<ParentMessage>> {
        self.request("ChainGetParentMessages", vec![helper::serialize(cid)])
            .await
    }

    async fn chain_get_tipset_by_height(
        &self,
        height: ChainEpoch,
        key: &TipsetKey,
    ) -> Result<Tipset> {
        self.request(
            "ChainGetTipSetByHeight",
            vec![helper::serialize(&height), helper::serialize(key)],
        )
        .await
    }

    async fn chain_read_obj(&self, cid: &Cid) -> Result<Vec<u8>> {
        let bytes: Bytes = self
            .request("ChainReadObj", vec![helper::serialize(cid)])
            .await?;
        Ok(bytes.into_inner())
    }

    async fn chain_has_obj(&self, cid: &Cid) -> Result<bool> {
        self.request("ChainHasObj", vec![helper::serialize(cid)])
            .await
    }

    async fn chain_stat_obj(&self, obj: &Cid, base: &Cid) -> Result<ObjStat> {
        self.request(
            "ChainHasObj",
            vec![helper::serialize(obj), helper::serialize(base)],
        )
        .await
    }

    async fn chain_set_head(&self, key: &TipsetKey) -> Result<()> {
        self.request("ChainSetHead", vec![helper::serialize(key)])
            .await
    }

    async fn chain_get_genesis(&self) -> Result<Tipset> {
        self.request("ChainGetGenesis", vec![]).await
    }

    async fn chain_tipset_weight(&self, key: &TipsetKey) -> Result<BigInt> {
        let bigint: BigIntWrapper = self
            .request("ChainTipSetWeight", vec![helper::serialize(key)])
            .await?;
        Ok(bigint.into_inner())
    }

    /*
    async fn chain_get_node(&self, path: &str) -> Result<IpldObject> {
        self.request("ChainGetNode", vec![helper::serialize(path)]).await
    }
    */

    async fn chain_get_message(&self, cid: &Cid) -> Result<UnsignedMessage> {
        self.request("ChainGetMessage", vec![helper::serialize(cid)])
            .await
    }

    async fn chain_get_path(&self, from: &TipsetKey, to: &TipsetKey) -> Result<Vec<HeadChange>> {
        self.request(
            "ChainGetPath",
            vec![helper::serialize(from), helper::serialize(to)],
        )
        .await
    }

    async fn chain_export(
        &self,
        key: &TipsetKey,
    ) -> Result<(SubscriptionId, NotificationStream<Bytes>)> {
        self.subscribe("ChainExport", vec![helper::serialize(key)])
            .await
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HeadChangeType {
    Revert,
    Apply,
    Current,
}

#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeadChange {
    pub r#type: HeadChangeType,
    pub val: Tipset,
}

/*
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpldObject {
    cid: Cid,
    obj: IpldNode,
}
*/

#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockMessages {
    pub bls_messages: Vec<UnsignedMessage>,
    pub secpk_messages: Vec<SignedMessage>,
    pub cids: Vec<Cid>,
}

// Only For chain_get_parent_messages
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParentMessage {
    pub cid: Cid,
    pub message: UnsignedMessage,
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjStat {
    pub size: u64,
    pub links: u64,
}

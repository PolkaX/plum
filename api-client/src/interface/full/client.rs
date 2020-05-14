// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use libp2p_core::PeerId;
use serde::{Deserialize, Serialize};

use cid::Cid;
use plum_address::Address;
use plum_bigint::{bigint_json, BigInt};
use plum_piece::UnpaddedPieceSize;
// use plum_types::{ChainEpoch, DealId};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[doc(hidden)]
#[async_trait::async_trait]
pub trait ClientApi: RpcClient {
    // ClientImport imports file under the specified path into filestore
    async fn client_import(&self, r#ref: &FileRef) -> Result<Cid> {
        self.request("ClientImport", vec![helper::serialize(r#ref)])
            .await
    }

    /*
    async fn client_start_deal(&self, params: &StartDealParams) -> Result<Cid> {
        self.request("ClientStartDeal", vec![helper::serialize(params)])
            .await
    }

    async fn client_get_deal_info(&self, cid: &Cid) -> Result<DealInfo> {
        self.request("ClientGetDealInfo", vec![helper::serialize(cid)])
            .await
    }

    async fn client_list_deals(&self) -> Result<Vec<DealInfo>> {
        self.request("ClientListDeals", vec![]).await
    }
    */

    async fn client_has_local(&self, root: &Cid) -> Result<bool> {
        self.request("ClientHasLocal", vec![helper::serialize(root)])
            .await
    }

    async fn client_find_data(&self, root: &Cid) -> Result<Vec<QueryOffer>> {
        self.request("ClientFindData", vec![helper::serialize(root)])
            .await
    }

    async fn client_retrieve(&self, order: &RetrievalOrder, r#ref: &FileRef) -> Result<()> {
        self.request(
            "ClientFindData",
            vec![helper::serialize(order), helper::serialize(r#ref)],
        )
        .await
    }

    /*
    async fn client_query_ask(
        &self,
        peer_id: &PeerId,
        miner: &Address,
    ) -> Result<SignedStorageAsk> {
        self.request(
            "ClientQueryAsk",
            vec![
                helper::serialize_with(helper::peer_id::serialize, peer_id),
                helper::serialize(miner),
            ],
        )
        .await
    }
    */

    async fn client_calc_comm_p(&self, inpath: &str, miner: &Address) -> Result<CommPRet> {
        self.request(
            "ClientCalcCommP",
            vec![helper::serialize(&inpath), helper::serialize(miner)],
        )
        .await
    }

    async fn client_gen_car(&self, r#ref: &FileRef, outpath: &str) -> Result<()> {
        self.request(
            "ClientCalcCommP",
            vec![helper::serialize(r#ref), helper::serialize(&outpath)],
        )
        .await
    }

    /*
    async fn client_list_imports(&self) -> Result<Vec<Import>> {
        self.request("ClientListImports", vec![]).await
    }
    */
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileRef {
    pub path: String,
    #[serde(rename = "IsCAR")]
    pub is_car: bool,
}

/*
///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartDealParams {
    // pub data: storagemarket::DataRef,
    pub wallet: Address,
    pub miner: Address,
    #[serde(with = "bigint_json")]
    pub epoch_price: BigInt,
    pub min_blocks_duration: u64,
    deal_start_epoch: ChainEpoch,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Import {
    // pub status: filestore::Status,
    pub key: Cid,
    pub file_path: String,
    pub size: u64,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealInfo {
    pub proposal_cid: Cid,
    // pub state: storagemarket::StorageDealStatus,
    // more information about deal state, particularly errors
    pub message: String,
    pub provider: Address,

    #[serde(rename = "PieceCID")]
    pub piece_cid: Cid,
    pub size: u64,

    #[serde(with = "bigint_json")]
    pub price_per_epoch: BigInt,
    pub duration: u64,

    #[serde(rename = "DealID")]
    pub deal_id: DealId,
}
*/

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryOffer {
    pub err: String,

    pub root: Cid,

    pub size: u64,
    #[serde(with = "bigint_json")]
    pub min_price: BigInt,
    pub payment_interval: u64,
    pub payment_interval_increase: u64,
    pub miner: Address,
    #[serde(rename = "MinerPeerID")]
    #[serde(with = "crate::helper::peer_id")]
    pub miner_peer_id: PeerId,
}

impl QueryOffer {
    ///
    pub fn order(&self, client: Address) -> RetrievalOrder {
        RetrievalOrder {
            root: self.root.clone(),
            size: self.size,
            total: self.min_price.clone(),
            payment_interval: self.payment_interval,
            payment_interval_increase: self.payment_interval_increase,
            client,

            miner: self.miner.clone(),
            miner_peer_id: self.miner_peer_id.clone(),
        }
    }
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RetrievalOrder {
    // TODO: make this less unixfs specific
    pub root: Cid,
    pub size: u64,
    // TODO: support offset
    #[serde(with = "bigint_json")]
    pub total: BigInt,
    pub payment_interval: u64,
    pub payment_interval_increase: u64,
    pub client: Address,
    pub miner: Address,
    #[serde(rename = "MinerPeerID")]
    #[serde(with = "crate::helper::peer_id")]
    pub miner_peer_id: PeerId,
}

///
#[doc(hidden)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommPRet {
    pub root: Cid,
    pub size: UnpaddedPieceSize,
}

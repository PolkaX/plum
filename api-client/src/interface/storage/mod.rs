// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod types;

pub use self::types::*;

use std::collections::HashMap;

use cid::Cid;
use plum_address::Address;
use plum_bigint::{BigInt, BigIntRefWrapper};
use plum_sector::{SectorNumber, SectorSize};
use plum_tipset::Tipset;

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

/// The StorageMiner API Interface, which is a low-level interface to the
/// Filecoin network storage miner node.
#[doc(hidden)]
#[async_trait::async_trait]
pub trait StorageMinerApi: RpcClient {
    async fn actor_address(&self) -> Result<Address> {
        self.request("ActorAddress", vec![]).await
    }

    async fn actor_sector_size(&self, addr: &Address) -> Result<SectorSize> {
        self.request("ActorSectorSize", vec![helper::serialize(addr)])
            .await
    }

    async fn mining_base(&self) -> Result<Tipset> {
        self.request("MiningBase", vec![]).await
    }

    // Temp api for testing
    async fn pledge_sector(&self) -> Result<()> {
        self.request("PledgeSector", vec![]).await
    }

    /// Get the status of a given sector by ID
    async fn sectors_status(&self, number: SectorNumber) -> Result<SectorInfo> {
        self.request("SectorsStatus", vec![helper::serialize(&number)])
            .await
    }

    // List all staged sectors
    async fn sectors_list(&self) -> Result<Vec<SectorNumber>> {
        self.request("SectorsList", vec![]).await
    }

    async fn sectors_refs(&self) -> Result<HashMap<String, Vec<SealedRef>>> {
        self.request("SectorsRefs", vec![]).await
    }

    async fn sectors_update(&self, number: SectorNumber, state: &str) -> Result<()> {
        self.request(
            "SectorsUpdate",
            vec![helper::serialize(&number), helper::serialize(&state)],
        )
        .await
    }

    /*
    async fn storage_list(&self) -> Result<HashMap<stores::ID, stores::Decl>> {
        self.request("StorageList", vec![]).await
    }
    async fn storage_local(&self) -> Result<HashMap<stores::ID, String>> {
        self.request("StorageLocal", vec![]).await
    }
    async fn storage_stat(&self, id: stores::ID) -> Result<stores::FsStat> {
        self.request("StorageStat", vec![id]).await
    }
    */

    // WorkerConnect tells the node to connect to workers RPC
    async fn worker_connect(&self, s: &str) -> Result<()> {
        self.request("WorkerConnect", vec![helper::serialize(&s)])
            .await
    }
    /*
    async fn worker_stats(&self) -> Result<HashMap<u64, storiface::WorkerStats>> {
        self.request("WorkStats", vec![]).await
    }
    */

    async fn market_import_deal_data(&self, prop_cid: &Cid, path: &str) -> Result<()> {
        self.request(
            "MarketImportDealData",
            vec![helper::serialize(prop_cid), helper::serialize(&path)],
        )
        .await
    }
    /*
    async fn market_list_deals(&self) -> Result<Vec<storagemarket::StorageDeal>> {
        self.request("MarketListDeals", vec![]).await
    }

    async fn market_list_incomplete_deals(&self) -> Result<Vec<storagemarket::MinerDeal>> {
        self.request("MarketListIncompleteDeals", vec![]).await
    }
    */
    async fn market_set_price(&self, price: &BigInt) -> Result<()> {
        self.request(
            "MarketSetPrice",
            vec![helper::serialize(&BigIntRefWrapper::from(price))],
        )
        .await
    }

    async fn deals_import_data(&self, deal_prop_cid: &Cid, file: &str) -> Result<()> {
        self.request(
            "DealsImportData",
            vec![helper::serialize(deal_prop_cid), helper::serialize(&file)],
        )
        .await
    }
    /*
    async fn deals_list(&self) -> Result<Vec<storagemarket::StorageDeal>> {
        self.request("DealsList", vec![]).await
    }
    */

    async fn storage_add_local(&self, path: &str) -> Result<()> {
        self.request("StorageAddLocal", vec![helper::serialize(&path)])
            .await
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod types;

pub use self::types::*;

use libp2p_core::PeerId;

use plum_bytes::Bytes;
use plum_peerid::{PeerIdRefWrapper, PeerIdWrapper};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

/// The Common API interface
#[doc(hidden)]
#[async_trait::async_trait]
pub trait CommonApi: RpcClient {
    async fn auth_verify(&self, token: &str) -> Result<Vec<Permission>> {
        self.request("AuthVerify", vec![helper::serialize(&token)])
            .await
    }

    async fn auth_new(&self, permissions: &[Permission]) -> Result<Vec<u8>> {
        let bytes: Bytes = self
            .request("AuthNew", vec![helper::serialize(&permissions)])
            .await?;
        Ok(bytes.into_inner())
    }

    async fn net_connectedness(&self, peer_id: &PeerId) -> Result<Connectedness> {
        self.request(
            "NetConnectedness",
            vec![helper::serialize(&PeerIdRefWrapper::from(peer_id))],
        )
        .await
    }

    async fn net_peers(&self) -> Result<Vec<PeerAddrInfo>> {
        self.request("NetPeers", vec![]).await
    }

    async fn net_connect(&self, addr_info: &PeerAddrInfo) -> Result<()> {
        self.request("NetConnect", vec![helper::serialize(addr_info)])
            .await
    }

    async fn net_addrs_listen(&self) -> Result<PeerAddrInfo> {
        self.request("NetAddrsListen", vec![]).await
    }

    async fn net_disconnect(&self, peer_id: &PeerId) -> Result<()> {
        self.request(
            "NetDisconnect",
            vec![helper::serialize(&PeerIdRefWrapper::from(peer_id))],
        )
        .await
    }

    async fn net_find_peer(&self, peer_id: &PeerId) -> Result<PeerAddrInfo> {
        self.request(
            "NetFindPeer",
            vec![helper::serialize(&PeerIdRefWrapper::from(peer_id))],
        )
        .await
    }

    // returns peer id of libp2p node backing this API.
    async fn id(&self) -> Result<PeerId> {
        let peer_id: PeerIdWrapper = self.request("ID", vec![]).await?;
        Ok(peer_id.into_inner())
    }

    // provides information about API provider.
    async fn version(&self) -> Result<Version> {
        self.request("Version", vec![]).await
    }

    async fn log_list(&self) -> Result<Vec<String>> {
        self.request("LogList", vec![]).await
    }

    async fn log_set_level(&self, subsystem: &str, level: &str) -> Result<()> {
        self.request(
            "LogSetLevel",
            vec![helper::serialize(&subsystem), helper::serialize(&level)],
        )
        .await
    }

    // trigger graceful shutdown
    async fn shutdown(&self) -> Result<()> {
        self.request("Shutdown", vec![]).await
    }
}

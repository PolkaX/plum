// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use cid::Cid;
use plum_block::BlockHeader;
use plum_message::{SignedMessage, UnsignedMessage};

pub const BLOCKSYNC_PROTOCOL_ID: &[u8] = b"/fil/sync/blk/0.0.1";

/// The BlockSync request, see lotus/chain/blocksync/blocksync.go
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSyncRequest {
    /// The tipset to start sync from
    pub start: Vec<Cid>,
    /// The amount of epochs to sync by
    pub request_length: u64,
    /// 1 = Block only, 2 = Messages only, 3 = Blocks and Messages
    pub options: u64,
}

impl ser::Serialize for BlockSyncRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (&self.start, &self.request_length, &self.options).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for BlockSyncRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (start, request_length, options) = de::Deserialize::deserialize(deserializer)?;
        Ok(BlockSyncRequest {
            start,
            request_length,
            options,
        })
    }
}

/// The response to a BlockSync request.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSyncResponse {
    /// The tipsets requested
    pub chain: Vec<BlockSyncTipset>,
    /// Error code
    pub status: u64,
    /// Status message indicating failure reason
    pub message: String,
}

impl ser::Serialize for BlockSyncResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (&self.chain, &self.status, &self.message).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for BlockSyncResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (chain, status, message) = de::Deserialize::deserialize(deserializer)?;
        Ok(BlockSyncResponse {
            chain,
            status,
            message,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockSyncTipset {
    /// The blocks in the tipset
    pub blocks: Vec<BlockHeader>,

    /// Signed bls messages
    pub bls_msgs: Vec<UnsignedMessage>,
    /// Describes which block each message belongs to
    pub bls_msg_includes: Vec<Vec<u64>>,

    /// Unsigned secp messages
    pub secp_msgs: Vec<SignedMessage>,
    /// Describes which block each message belongs to
    pub secp_msg_includes: Vec<Vec<u64>>,
}

impl ser::Serialize for BlockSyncTipset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (
            &self.blocks,
            &self.bls_msgs,
            &self.bls_msg_includes,
            &self.secp_msgs,
            &self.secp_msg_includes,
        )
            .serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for BlockSyncTipset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (blocks, bls_msgs, bls_msg_includes, secp_msgs, secp_msg_includes) =
            de::Deserialize::deserialize(deserializer)?;
        Ok(BlockSyncTipset {
            blocks,
            bls_msgs,
            bls_msg_includes,
            secp_msgs,
            secp_msg_includes,
        })
    }
}

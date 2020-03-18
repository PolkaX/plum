// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use plum_block::BlockHeader;

use crate::key::TipsetKey;

///
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Tipset {
    key: TipsetKey,
    blocks: Vec<BlockHeader>,
    height: u64,
}

impl Tipset {
    ///
    pub fn new(key: TipsetKey, blocks: Vec<BlockHeader>) -> Self {
        todo!()
    }

    /// Return the key.
    pub fn key(&self) -> &TipsetKey {
        &self.key
    }

    /// Return the CIDs of blocks.
    pub fn cids(&self) -> &[Cid] {
        self.key.cids()
    }

    ///
    pub fn blocks(&self) -> &[BlockHeader] {
        &self.blocks
    }

    ///
    pub fn height(&self) -> u64 {
        self.height
    }
}

/// TipsetKey CBOR serialization/deserialization, need to use `serde_cbor::Serializer` and `serde_cbor::Deserializer`
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_block::{block_header_cbor, BlockHeader};

    use super::Tipset;
    use crate::key::TipsetKey;

    #[derive(Serialize)]
    struct CborBlockHeaderRef<'a>(#[serde(with = "block_header_cbor")] &'a BlockHeader);
    #[derive(Serialize)]
    struct TupleTipsetRef<'a>(
        #[serde(with = "crate::key::cbor")] &'a TipsetKey,
        &'a [CborBlockHeaderRef<'a>],
        &'a u64,
    );

    /// CBOR serialization.
    pub fn serialize<S>(tipset: &Tipset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let blocks = tipset
            .blocks
            .iter()
            .map(|block| CborBlockHeaderRef(block))
            .collect::<Vec<_>>();
        TupleTipsetRef(&tipset.key, &blocks, &tipset.height).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborBlockHeader(#[serde(with = "block_header_cbor")] BlockHeader);
    #[derive(Deserialize)]
    struct TupleTipset(
        #[serde(with = "crate::key::cbor")] TipsetKey,
        Vec<CborBlockHeader>,
        u64,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Tipset, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleTipset(key, blocks, height) = TupleTipset::deserialize(deserializer)?;
        let blocks = blocks.into_iter().map(|block| block.0).collect();
        Ok(Tipset {
            key,
            blocks,
            height,
        })
    }
}

/// TipsetKey JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_block::{block_header_json, BlockHeader};

    use super::Tipset;
    use crate::key::TipsetKey;

    #[derive(Serialize)]
    struct JsonBlockHeaderRef<'a>(#[serde(with = "block_header_json")] &'a BlockHeader);
    #[derive(Serialize)]
    struct TupleTipsetRef<'a>(
        #[serde(with = "crate::key::json")] &'a TipsetKey,
        &'a [JsonBlockHeaderRef<'a>],
        &'a u64,
    );

    /// CBOR serialization.
    pub fn serialize<S>(tipset: &Tipset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let blocks = tipset
            .blocks
            .iter()
            .map(|block| JsonBlockHeaderRef(block))
            .collect::<Vec<_>>();
        TupleTipsetRef(&tipset.key, &blocks, &tipset.height).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonBlockHeader(#[serde(with = "block_header_json")] BlockHeader);
    #[derive(Deserialize)]
    struct TupleTipset(
        #[serde(with = "crate::key::json")] TipsetKey,
        Vec<JsonBlockHeader>,
        u64,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Tipset, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleTipset(key, blocks, height) = TupleTipset::deserialize(deserializer)?;
        let blocks = blocks.into_iter().map(|block| block.0).collect();
        Ok(Tipset {
            key,
            blocks,
            height,
        })
    }
}

#[cfg(test)]
mod tests {}

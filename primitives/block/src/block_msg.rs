// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use serde::{de, ser};

use crate::header::BlockHeader;

/// The block message.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct BlockMsg {
    /// The block header.
    pub header: BlockHeader,
    /// The CIDs of `BLS` messages.
    pub bls_messages: Vec<Cid>,
    /// The CIDs of `Secp256k1` messages.
    pub secpk_messages: Vec<Cid>,
}

impl BlockMsg {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        self.header.cid()
    }
}

impl ser::Serialize for BlockMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for BlockMsg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// BlockMsg CBOR serialization/deserialization
pub mod cbor {
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

    use super::BlockMsg;
    use crate::header::BlockHeader;

    #[derive(Serialize)]
    struct CborCidRef<'a>(#[serde(with = "cid::ipld_dag_cbor")] &'a Cid);
    #[derive(Serialize)]
    struct CborBlockMsgRef<'a>(
        #[serde(with = "crate::header::cbor")] &'a BlockHeader,
        &'a [CborCidRef<'a>],
        &'a [CborCidRef<'a>],
    );

    /// CBOR serialization
    pub fn serialize<S>(block: &BlockMsg, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborBlockMsgRef(
            &block.header,
            &block
                .bls_messages
                .iter()
                .map(|cid| CborCidRef(cid))
                .collect::<Vec<_>>(),
            &block
                .secpk_messages
                .iter()
                .map(|cid| CborCidRef(cid))
                .collect::<Vec<_>>(),
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborCid(#[serde(with = "cid::ipld_dag_cbor")] Cid);
    #[derive(Deserialize)]
    struct CborBlockMsg(
        #[serde(with = "crate::header::cbor")] BlockHeader,
        Vec<CborCid>,
        Vec<CborCid>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockMsg, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborBlockMsg(header, bls_msgs, secpk_msgs) = CborBlockMsg::deserialize(deserializer)?;
        Ok(BlockMsg {
            header,
            bls_messages: bls_msgs.into_iter().map(|cid| cid.0).collect(),
            secpk_messages: secpk_msgs.into_iter().map(|cid| cid.0).collect(),
        })
    }
}

/// BlockMsg JSON serialization/deserialization
pub mod json {
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

    use super::BlockMsg;
    use crate::header::BlockHeader;

    #[derive(Serialize)]
    struct JsonCidRef<'a>(#[serde(with = "cid::ipld_dag_json")] &'a Cid);
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockMsgRef<'a> {
        #[serde(with = "crate::header::json")]
        header: &'a BlockHeader,
        bls_messages: &'a [JsonCidRef<'a>],
        secpk_messages: &'a [JsonCidRef<'a>],
    }

    /// JSON serialization
    pub fn serialize<S>(block: &BlockMsg, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonBlockMsgRef {
            header: &block.header,
            bls_messages: &block
                .bls_messages
                .iter()
                .map(|cid| JsonCidRef(cid))
                .collect::<Vec<_>>(),
            secpk_messages: &block
                .secpk_messages
                .iter()
                .map(|cid| JsonCidRef(cid))
                .collect::<Vec<_>>(),
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonCid(#[serde(with = "cid::ipld_dag_json")] Cid);
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockMsg {
        #[serde(with = "crate::header::json")]
        header: BlockHeader,
        bls_messages: Vec<JsonCid>,
        secpk_messages: Vec<JsonCid>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockMsg, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonBlockMsg {
            header,
            bls_messages,
            secpk_messages,
        } = JsonBlockMsg::deserialize(deserializer)?;
        Ok(BlockMsg {
            header,
            bls_messages: bls_messages.into_iter().map(|cid| cid.0).collect(),
            secpk_messages: secpk_messages.into_iter().map(|cid| cid.0).collect(),
        })
    }
}

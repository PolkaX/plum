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
    use cid::{ipld_dag_cbor as cid_cbor, Cid};
    use serde::{de, ser, Deserialize, Serialize};

    use super::BlockMsg;
    use crate::header::BlockHeader;

    #[derive(Serialize)]
    struct CborBlockMsgRef<'a>(
        #[serde(with = "crate::header::cbor")] &'a BlockHeader,
        #[serde(with = "cid_cbor::vec")] &'a [Cid],
        #[serde(with = "cid_cbor::vec")] &'a [Cid],
    );

    /// CBOR serialization
    pub fn serialize<S>(block: &BlockMsg, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborBlockMsgRef(&block.header, &block.bls_messages, &block.secpk_messages)
            .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborBlockMsg(
        #[serde(with = "crate::header::cbor")] BlockHeader,
        #[serde(with = "cid_cbor::vec")] Vec<Cid>,
        #[serde(with = "cid_cbor::vec")] Vec<Cid>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockMsg, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborBlockMsg(header, bls_messages, secpk_messages) =
            CborBlockMsg::deserialize(deserializer)?;
        Ok(BlockMsg {
            header,
            bls_messages,
            secpk_messages,
        })
    }
}

/// BlockMsg JSON serialization/deserialization
pub mod json {
    use cid::{ipld_dag_json as cid_json, Cid};
    use serde::{de, ser, Deserialize, Serialize};

    use super::BlockMsg;
    use crate::header::BlockHeader;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockMsgRef<'a> {
        #[serde(with = "crate::header::json")]
        header: &'a BlockHeader,
        #[serde(with = "cid_json::vec")]
        bls_messages: &'a [Cid],
        #[serde(with = "cid_json::vec")]
        secpk_messages: &'a [Cid],
    }

    /// JSON serialization
    pub fn serialize<S>(block: &BlockMsg, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonBlockMsgRef {
            header: &block.header,
            bls_messages: &block.bls_messages,
            secpk_messages: &block.secpk_messages,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockMsg {
        #[serde(with = "crate::header::json")]
        header: BlockHeader,
        #[serde(with = "cid_json::vec")]
        bls_messages: Vec<Cid>,
        #[serde(with = "cid_json::vec")]
        secpk_messages: Vec<Cid>,
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
            bls_messages,
            secpk_messages,
        })
    }
}

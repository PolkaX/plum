// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use serde::{de, ser};

use plum_message::{SignedMessage, UnsignedMessage};

use crate::header::BlockHeader;

/// The complete block.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Block {
    /// The block header.
    pub header: BlockHeader,
    /// The `BLS` messages.
    pub bls_messages: Vec<UnsignedMessage>,
    /// The `Secp256k1` messages.
    pub secpk_messages: Vec<SignedMessage>,
}

impl Block {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        self.header.cid()
    }
}

impl ser::Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// Block CBOR serialization/deserialization
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_message::{
        signed_message_cbor, unsigned_message_cbor, SignedMessage, UnsignedMessage,
    };

    use super::Block;
    use crate::header::BlockHeader;

    #[derive(Serialize)]
    struct CborBlockRef<'a>(
        #[serde(with = "crate::header::cbor")] &'a BlockHeader,
        #[serde(with = "unsigned_message_cbor::vec")] &'a [UnsignedMessage],
        #[serde(with = "signed_message_cbor::vec")] &'a [SignedMessage],
    );

    /// CBOR serialization
    pub fn serialize<S>(block: &Block, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborBlockRef(&block.header, &block.bls_messages, &block.secpk_messages)
            .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborBlock(
        #[serde(with = "crate::header::cbor")] BlockHeader,
        #[serde(with = "unsigned_message_cbor::vec")] Vec<UnsignedMessage>,
        #[serde(with = "signed_message_cbor::vec")] Vec<SignedMessage>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Block, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborBlock(header, bls_messages, secpk_messages) = CborBlock::deserialize(deserializer)?;
        Ok(Block {
            header,
            bls_messages,
            secpk_messages,
        })
    }
}

/// Block JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_message::{
        signed_message_json, unsigned_message_json, SignedMessage, UnsignedMessage,
    };

    use super::Block;
    use crate::header::BlockHeader;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockRef<'a> {
        #[serde(with = "crate::header::json")]
        header: &'a BlockHeader,
        #[serde(with = "unsigned_message_json::vec")]
        bls_messages: &'a [UnsignedMessage],
        #[serde(with = "signed_message_json::vec")]
        secpk_messages: &'a [SignedMessage],
    }

    /// JSON serialization
    pub fn serialize<S>(block: &Block, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonBlockRef {
            header: &block.header,
            bls_messages: &block.bls_messages,
            secpk_messages: &block.secpk_messages,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlock {
        #[serde(with = "crate::header::json")]
        header: BlockHeader,
        #[serde(with = "unsigned_message_json::vec")]
        bls_messages: Vec<UnsignedMessage>,
        #[serde(with = "signed_message_json::vec")]
        secpk_messages: Vec<SignedMessage>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Block, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonBlock {
            header,
            bls_messages,
            secpk_messages,
        } = JsonBlock::deserialize(deserializer)?;
        Ok(Block {
            header,
            bls_messages,
            secpk_messages,
        })
    }
}

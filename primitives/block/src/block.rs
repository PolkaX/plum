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
    struct CborUnsignedMessageRef<'a>(#[serde(with = "unsigned_message_cbor")] &'a UnsignedMessage);
    #[derive(Serialize)]
    struct CborSignedMessageRef<'a>(#[serde(with = "signed_message_cbor")] &'a SignedMessage);
    #[derive(Serialize)]
    struct CborBlockRef<'a>(
        #[serde(with = "crate::header::cbor")] &'a BlockHeader,
        &'a [CborUnsignedMessageRef<'a>],
        &'a [CborSignedMessageRef<'a>],
    );

    /// CBOR serialization
    pub fn serialize<S>(block: &Block, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborBlockRef(
            &block.header,
            &block
                .bls_messages
                .iter()
                .map(|bls_msg| CborUnsignedMessageRef(bls_msg))
                .collect::<Vec<_>>(),
            &block
                .secpk_messages
                .iter()
                .map(|secp_msg| CborSignedMessageRef(secp_msg))
                .collect::<Vec<_>>(),
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborUnsignedMessage(#[serde(with = "unsigned_message_cbor")] UnsignedMessage);
    #[derive(Deserialize)]
    struct CborSignedMessage(#[serde(with = "signed_message_cbor")] SignedMessage);
    #[derive(Deserialize)]
    struct CborBlock(
        #[serde(with = "crate::header::cbor")] BlockHeader,
        Vec<CborUnsignedMessage>,
        Vec<CborSignedMessage>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Block, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborBlock(header, bls_msgs, secpk_msgs) = CborBlock::deserialize(deserializer)?;
        Ok(Block {
            header,
            bls_messages: bls_msgs.into_iter().map(|bls_msg| bls_msg.0).collect(),
            secpk_messages: secpk_msgs.into_iter().map(|secp_msg| secp_msg.0).collect(),
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
    struct JsonUnsignedMessageRef<'a>(#[serde(with = "unsigned_message_json")] &'a UnsignedMessage);
    #[derive(Serialize)]
    struct JsonSignedMessageRef<'a>(#[serde(with = "signed_message_json")] &'a SignedMessage);
    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockRef<'a> {
        #[serde(with = "crate::header::json")]
        header: &'a BlockHeader,
        bls_messages: &'a [JsonUnsignedMessageRef<'a>],
        secpk_messages: &'a [JsonSignedMessageRef<'a>],
    }

    /// JSON serialization
    pub fn serialize<S>(block: &Block, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonBlockRef {
            header: &block.header,
            bls_messages: &block
                .bls_messages
                .iter()
                .map(|bls_msg| JsonUnsignedMessageRef(bls_msg))
                .collect::<Vec<_>>(),
            secpk_messages: &block
                .secpk_messages
                .iter()
                .map(|secpk_msg| JsonSignedMessageRef(secpk_msg))
                .collect::<Vec<_>>(),
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonUnsignedMessage(#[serde(with = "unsigned_message_json")] UnsignedMessage);
    #[derive(Deserialize)]
    struct JsonSignedMessage(#[serde(with = "signed_message_json")] SignedMessage);
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlock {
        #[serde(with = "crate::header::json")]
        header: BlockHeader,
        bls_messages: Vec<JsonUnsignedMessage>,
        secpk_messages: Vec<JsonSignedMessage>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Block, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonBlock {
            header,
            bls_messages: bls_msgs,
            secpk_messages: secp_msgs,
        } = JsonBlock::deserialize(deserializer)?;
        Ok(Block {
            header,
            bls_messages: bls_msgs.into_iter().map(|bls_msg| bls_msg.0).collect(),
            secpk_messages: secp_msgs.into_iter().map(|secpk_msg| secpk_msg.0).collect(),
        })
    }
}

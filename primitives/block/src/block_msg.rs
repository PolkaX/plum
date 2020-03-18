// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use serde::{de, ser};

use crate::header::BlockHeader;

/// The block message.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct BlockMsg {
    /// The block header.
    pub header: BlockHeader,
    /// The CIDs of bls messages.
    pub bls_msgs: Vec<Cid>,
    /// The CIDs of secp256k1 messages.
    pub secp_msgs: Vec<Cid>,
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
    struct TupleBlockMsgRef<'a>(
        #[serde(with = "crate::header::cbor")] &'a BlockHeader,
        &'a [CborCidRef<'a>],
        &'a [CborCidRef<'a>],
    );

    /// CBOR serialization
    pub fn serialize<S>(block: &BlockMsg, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let bls_msgs = block
            .bls_msgs
            .iter()
            .map(|cid| CborCidRef(cid))
            .collect::<Vec<_>>();
        let secp_msgs = block
            .secp_msgs
            .iter()
            .map(|cid| CborCidRef(cid))
            .collect::<Vec<_>>();
        TupleBlockMsgRef(&block.header, &bls_msgs, &secp_msgs).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborCid(#[serde(with = "cid::ipld_dag_cbor")] Cid);
    #[derive(Deserialize)]
    struct TupleBlockMsg(
        #[serde(with = "crate::header::cbor")] BlockHeader,
        Vec<CborCid>,
        Vec<CborCid>,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockMsg, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleBlockMsg(header, bls_msgs, secp_msgs) = TupleBlockMsg::deserialize(deserializer)?;
        let bls_msgs = bls_msgs.into_iter().map(|cid| cid.0).collect();
        let secp_msgs = secp_msgs.into_iter().map(|cid| cid.0).collect();
        Ok(BlockMsg {
            header,
            bls_msgs,
            secp_msgs,
        })
    }
}

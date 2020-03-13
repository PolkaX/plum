// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use cid::Cid;

///
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct MsgMeta {
    ///
    pub bls_msg: Cid,
    ///
    pub secp_msg: Cid,
}

impl ser::Serialize for MsgMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for MsgMeta {
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

    use cid::Cid;

    use super::MsgMeta;

    #[derive(Serialize)]
    struct TupleMsgMetaRef<'a>(
        #[serde(with = "cid::ipld_dag_cbor")] &'a Cid,
        #[serde(with = "cid::ipld_dag_cbor")] &'a Cid,
    );

    /// CBOR serialization
    pub fn serialize<S>(msg_meta: &MsgMeta, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        TupleMsgMetaRef(&msg_meta.bls_msg, &msg_meta.secp_msg).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct TupleMsgMeta(
        #[serde(with = "cid::ipld_dag_cbor")] Cid,
        #[serde(with = "cid::ipld_dag_cbor")] Cid,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<MsgMeta, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleMsgMeta(bls_msg, secp_msg) = TupleMsgMeta::deserialize(deserializer)?;
        Ok(MsgMeta { bls_msg, secp_msg })
    }
}

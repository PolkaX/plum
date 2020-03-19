// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec};
use serde::{de, ser};

///
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct MsgMeta {
    ///
    pub bls_msg: Cid,
    ///
    pub secp_msg: Cid,
}

impl MsgMeta {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        let data =
            serde_cbor::to_vec(self).expect("CBOR serialization of MsgMeta shouldn't be failed");
        self.cid_with_data(data)
    }

    /// Convert to the CID with the given CBOR serialized data of MsgData.
    ///
    /// For cases where serialized data of the MsgData is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        let hash = multihash::Blake2b256::digest(data.as_ref());
        Cid::new_v1(Codec::DagCBOR, hash)
    }
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
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

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

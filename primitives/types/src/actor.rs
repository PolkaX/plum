// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use serde::{de, ser};

use plum_bigint::BigInt;

///
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Actor {
    ///
    pub code: Cid,
    ///
    pub head: Cid,
    ///
    pub nonce: u64,
    ///
    pub balance: BigInt,
}

impl ser::Serialize for Actor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for Actor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// Actor CBOR serialization/deserialization
pub mod cbor {
    use cid::{ipld_dag_cbor as cid_cbor, Cid};
    use serde::{de, ser, Deserialize, Serialize};

    use plum_bigint::{bigint_cbor, BigInt};

    use super::Actor;

    #[derive(Serialize)]
    struct CborActorRef<'a>(
        #[serde(with = "cid_cbor")] &'a Cid,
        #[serde(with = "cid_cbor")] &'a Cid,
        &'a u64,
        #[serde(with = "bigint_cbor")] &'a BigInt,
    );

    /// CBOR serialization
    pub fn serialize<S>(actor: &Actor, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborActorRef(&actor.code, &actor.head, &actor.nonce, &actor.balance).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborActor(
        #[serde(with = "cid_cbor")] Cid,
        #[serde(with = "cid_cbor")] Cid,
        u64,
        #[serde(with = "bigint_cbor")] BigInt,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Actor, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborActor(code, head, nonce, balance) = CborActor::deserialize(deserializer)?;
        Ok(Actor {
            code,
            head,
            nonce,
            balance,
        })
    }
}

/// Actor JSON serialization/deserialization
pub mod json {
    use cid::{ipld_dag_json as cid_json, Cid};
    use serde::{de, ser, Deserialize, Serialize};

    use plum_bigint::{bigint_json, BigInt};

    use super::Actor;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonActorRef<'a> {
        #[serde(with = "cid_json")]
        code: &'a Cid,
        #[serde(with = "cid_json")]
        head: &'a Cid,
        nonce: &'a u64,
        #[serde(with = "bigint_json")]
        balance: &'a BigInt,
    }

    /// JSON serialization
    pub fn serialize<S>(actor: &Actor, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonActorRef {
            code: &actor.code,
            head: &actor.head,
            nonce: &actor.nonce,
            balance: &actor.balance,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonActor {
        #[serde(with = "cid_json")]
        code: Cid,
        #[serde(with = "cid_json")]
        head: Cid,
        nonce: u64,
        #[serde(with = "bigint_json")]
        balance: BigInt,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Actor, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonActor {
            code,
            head,
            nonce,
            balance,
        } = JsonActor::deserialize(deserializer)?;
        Ok(Actor {
            code,
            head,
            nonce,
            balance,
        })
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser, Deserialize, Serialize};

use cid::Cid;
use plum_bigint::BigInt;

pub const HELLO_PROTOCOL_ID: &[u8] = b"/fil/hello/1.0.0";

/// The Hello request, see lotus/node/hello/hello.go
#[derive(Clone, Debug, PartialEq)]
pub struct HelloRequest {
    pub heaviest_tip_set: Vec<Cid>,
    pub heaviest_tipset_weight: BigInt,
    pub genesis_hash: Cid,
}

#[derive(Serialize)]
struct CborBigIntRef<'a>(#[serde(with = "plum_bigint::bigint_json")] &'a BigInt);
#[derive(Deserialize)]
struct CborBigInt(#[serde(with = "plum_bigint::bigint_json")] BigInt);

impl ser::Serialize for HelloRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (
            &self.heaviest_tip_set,
            CborBigIntRef(&self.heaviest_tipset_weight),
            &self.genesis_hash,
        )
            .serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for HelloRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (heaviest_tip_set, heaviest_tipset_weight, genesis_hash) =
            <(Vec<Cid>, CborBigInt, Cid)>::deserialize(deserializer)?;

        Ok(HelloRequest {
            heaviest_tip_set,
            heaviest_tipset_weight: heaviest_tipset_weight.0,
            genesis_hash,
        })
    }
}

/// The response to a Hello request.
#[derive(Clone, Debug, PartialEq)]
pub struct HelloResponse {
    /// Time of arrival in unix nanoseconds
    pub arrival: i64,
    /// Time sent in unix nanoseconds
    pub sent: i64,
}

impl ser::Serialize for HelloResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (&self.arrival, &self.sent).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for HelloResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (arrival, sent) = de::Deserialize::deserialize(deserializer)?;
        Ok(HelloResponse { arrival, sent })
    }
}

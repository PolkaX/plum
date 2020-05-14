// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_json::{
    value::Serializer as JsonValueSerializer, Error as JsonError, Value as JsonValue,
};

#[inline]
pub fn serialize<T: Serialize>(value: &T) -> JsonValue {
    value
        .serialize(JsonValueSerializer)
        .expect("Types never fail to serialize")
}

#[inline]
pub fn serialize_with<F, T>(f: F, value: &T) -> JsonValue
where
    F: Fn(&T, JsonValueSerializer) -> Result<JsonValue, JsonError>,
{
    f(value, JsonValueSerializer).expect("Types never fail to serialize")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerIdWrapper(#[serde(with = "self::peer_id")] libp2p_core::PeerId);

impl PeerIdWrapper {
    /// Consumes the wrapper, returning the underlying libp2p_core::PeerId.
    pub fn into_inner(self) -> libp2p_core::PeerId {
        self.0
    }
}

/// PeerId JSON serialization/deserialization
pub mod peer_id {
    use libp2p_core::PeerId;
    use serde::{de, ser, Deserialize, Serialize};

    /// JSON serialization
    pub fn serialize<S>(peer_id: &PeerId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        peer_id.to_string().serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<PeerId, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let peer_id = String::deserialize(deserializer)?
            .parse::<libp2p_core::PeerId>()
            .map_err(|err| de::Error::custom(err.to_string()))?;
        Ok(peer_id)
    }
}

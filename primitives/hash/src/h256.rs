// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use fixed_hash::construct_fixed_hash;

construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    pub struct H256(32);
}

/// Serialization/deserialization of H256.
pub mod raw {
    use serde::{de, ser};
    use serde_bytes::{ByteBuf, Bytes, Deserialize, Serialize};

    use super::H256;

    /// Serialization of H256.
    pub fn serialize<S>(h256: &H256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        Bytes::new(h256.as_bytes()).serialize(serializer)
    }

    /// Deserialization of H256.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<H256, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes = ByteBuf::deserialize(deserializer)?;
        if bytes.len() == H256::len_bytes() {
            Ok(H256::from_slice(bytes.as_slice()))
        } else {
            Err(de::Error::custom("H256 length must be 32 Bytes"))
        }
    }
}

/// Serialization/deserialization of Option<H256>.
pub mod option {
    use serde::{de, ser};
    use serde_bytes::{ByteBuf, Bytes, Deserialize, Serialize};

    use super::H256;

    /// Serialization of Option<H256>.
    pub fn serialize<S>(h256: &Option<H256>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        Bytes::new(
            h256.as_ref()
                .map(|hash| hash.as_bytes())
                .unwrap_or_default(),
        )
        .serialize(serializer)
    }

    /// Deserialization of Option<H256>.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<H256>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes = ByteBuf::deserialize(deserializer)?;
        if bytes.is_empty() {
            Ok(None)
        } else if bytes.len() == H256::len_bytes() {
            Ok(Some(H256::from_slice(bytes.as_slice())))
        } else {
            Err(de::Error::custom("H256 length must be 32 Bytes"))
        }
    }
}

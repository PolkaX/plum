// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;

use serde::{de, ser};

/// The Key type
#[derive(Eq, PartialEq, Clone)]
pub enum KeyType {
    /// secp256k1 key
    Secp256k1,
    /// bls key
    Bls,
    /// libp2p-host key
    Libp2pHost,
    /// jwt-hmac-secret key
    JwtHmacSecret,
    /// other key
    Other(String),
}

impl fmt::Debug for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyType::Secp256k1 => "secp256k1",
            KeyType::Bls => "bls",
            KeyType::Libp2pHost => "libp2p-host",
            KeyType::JwtHmacSecret => "jwt-hmac-secret",
            KeyType::Other(s) => s.as_str(),
        };
        f.write_str(s)
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ser::Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::key_type_json::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::key_type_json::deserialize(deserializer)
    }
}

/// KeyType JSON serialization/deserialization.
pub mod key_type_json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::KeyType;

    /// JSON serialization
    pub fn serialize<S>(key_type: &KeyType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        key_type.to_string().serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "secp256k1" => KeyType::Secp256k1,
            "bls" => KeyType::Bls,
            "libp2p-host" => KeyType::Libp2pHost,
            "jwt-hmac-secret" => KeyType::JwtHmacSecret,
            _ => KeyType::Other(s),
        })
    }
}

/// KeyInfo is used for storing keys in KeyStore.
#[derive(PartialEq, Clone, Debug)]
pub struct KeyInfo {
    /// The key type.
    pub ty: KeyType,
    /// The private key corresponding to key type.
    pub privkey: Vec<u8>,
}

impl ser::Serialize for KeyInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::key_info_json::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for KeyInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::key_info_json::deserialize(deserializer)
    }
}

/// KeyInfo JSON serialization/deserialization.
pub mod key_info_json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::{KeyInfo, KeyType};

    #[derive(Serialize)]
    struct JsonKeyInfoRef<'a> {
        #[serde(rename = "Type")]
        #[serde(with = "super::key_type_json")]
        ty: &'a KeyType,
        #[serde(rename = "PrivateKey")]
        #[serde(with = "plum_types::base64")]
        privkey: &'a [u8],
    }

    /// JSON serialization
    pub fn serialize<S>(key_info: &KeyInfo, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonKeyInfoRef {
            ty: &key_info.ty,
            privkey: &key_info.privkey,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonKeyInfo {
        #[serde(rename = "Type")]
        #[serde(with = "super::key_type_json")]
        ty: KeyType,
        #[serde(rename = "PrivateKey")]
        #[serde(with = "plum_types::base64")]
        privkey: Vec<u8>,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyInfo, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let key_info = JsonKeyInfo::deserialize(deserializer)?;
        Ok(KeyInfo {
            ty: key_info.ty,
            privkey: key_info.privkey,
        })
    }
}

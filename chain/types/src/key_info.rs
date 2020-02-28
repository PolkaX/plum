// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::{TryFrom, TryInto};
use std::fmt;
use thiserror::Error;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Error, Debug)]
pub enum KeyTypeError {
    #[error("unknown key type for signature. type number: {0}")]
    UnknownSignKeyType(u8),

    #[error("can't convert current `KeyType` into `SignKeyType`. KeyType:{0:?}")]
    UnsupportedSignKeyType(KeyType),
}

/// The type of key that store in keystore.
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
#[repr(u8)]
pub enum SignKeyType {
    /// SECP256K1 key.
    SECP256K1 = 1,
    /// BLS key.
    BLS = 2,
}

impl TryFrom<u8> for SignKeyType {
    type Error = KeyTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SignKeyType::SECP256K1),
            2 => Ok(SignKeyType::BLS),
            _ => Err(KeyTypeError::UnknownSignKeyType(value)),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum KeyType {
    SECP256K1,
    BLS,
    Libp2p,
    Other(String),
}

impl fmt::Debug for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyType::SECP256K1 => SECP256K1,
            KeyType::BLS => BLS,
            KeyType::Libp2p => LIBP2P,
            KeyType::Other(s) => s.as_str(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<SignKeyType> for KeyType {
    fn from(s: SignKeyType) -> Self {
        match s {
            SignKeyType::SECP256K1 => KeyType::SECP256K1,
            SignKeyType::BLS => KeyType::BLS,
        }
    }
}

impl TryInto<SignKeyType> for KeyType {
    type Error = KeyTypeError;

    fn try_into(self) -> Result<SignKeyType, Self::Error> {
        match self {
            KeyType::SECP256K1 => Ok(SignKeyType::SECP256K1),
            KeyType::BLS => Ok(SignKeyType::BLS),
            _ => Err(KeyTypeError::UnsupportedSignKeyType(self)),
        }
    }
}

pub const SECP256K1: &'static str = "secp256k1";
pub const BLS: &'static str = "bls";
pub const LIBP2P: &'static str = "libp2p-host";

impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            KeyType::SECP256K1 => SECP256K1.serialize(serializer),
            KeyType::BLS => BLS.serialize(serializer),
            KeyType::Libp2p => LIBP2P.serialize(serializer),
            KeyType::Other(s) => s.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let t = match s.as_str() {
            SECP256K1 => KeyType::SECP256K1,
            BLS => KeyType::BLS,
            LIBP2P => KeyType::Libp2p,
            _ => KeyType::Other(s),
        };
        Ok(t)
    }
}

/// KeyInfo is used for storing keys in KeyStore.
#[derive(Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// The key type.
    #[serde(rename = "Type")]
    pub ty: KeyType,
    /// The private key corresponding to key type.
    #[serde(
        rename = "PrivateKey",
        serialize_with = "serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub privkey: Vec<u8>,
}
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + AsRef<[u8]>,
    S: Serializer,
{
    String::from_utf8_lossy(bytes.as_ref()).serialize(serializer)
}

impl fmt::Debug for KeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KeyInfo {{ type: {:?}, priv: {:} }}",
            self.ty,
            String::from_utf8_lossy(&self.privkey)
        )
    }
}

impl fmt::Display for KeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[test]
fn test_keyinfo() {
    let raw = r#"{"Type":"secp256k1","PrivateKey":"hFSlqK6Wf83rYH+jpKSVCMMVg1V3IkRZvOllItxDI0A="}"#;
    let ki: KeyInfo = serde_json::from_str(raw).unwrap();
    assert_eq!(
        &format!("{:}", ki),
        "KeyInfo { type: secp256k1, priv: hFSlqK6Wf83rYH+jpKSVCMMVg1V3IkRZvOllItxDI0A= }"
    );
    let out = serde_json::to_string(&ki).unwrap();
    assert_eq!(out.as_str(), raw);

    let raw = r#"{"Type":"libp2p-host","PrivateKey":"CAESQHB5fBFkLwCDWG8Qx5WvQ44koHmHVagDiVu23SUS7Uk/+S/WPIh15CHF+JZyeS0XBnsMGXnKkgrgXfYys6ZRuQ8="}"#;
    let ki: KeyInfo = serde_json::from_str(raw).unwrap();
    assert_eq!(&format!("{:}", ki), "KeyInfo { type: libp2p-host, priv: CAESQHB5fBFkLwCDWG8Qx5WvQ44koHmHVagDiVu23SUS7Uk/+S/WPIh15CHF+JZyeS0XBnsMGXnKkgrgXfYys6ZRuQ8= }");
    let out = serde_json::to_string(&ki).unwrap();
    assert_eq!(out.as_str(), raw);

    let raw =
        r#"{"Type":"jwt-hmac-secret","PrivateKey":"bvL9+O7FWAhAerKsBwJoXnzNpIYVpZwUR8E0Y6vIYmE="}"#;
    let ki: KeyInfo = serde_json::from_str(raw).unwrap();
    assert_eq!(
        &format!("{:}", ki),
        "KeyInfo { type: jwt-hmac-secret, priv: bvL9+O7FWAhAerKsBwJoXnzNpIYVpZwUR8E0Y6vIYmE= }"
    );
    let out = serde_json::to_string(&ki).unwrap();
    assert_eq!(out.as_str(), raw);

    let raw = r#"{"Type":"bls","PrivateKey":"B9C+MmuLJ2yHTgZyHw74YAW9H6hYiYeQ6m0R+GTJsDA="}"#;
    let ki: KeyInfo = serde_json::from_str(raw).unwrap();
    assert_eq!(
        &format!("{:}", ki),
        "KeyInfo { type: bls, priv: B9C+MmuLJ2yHTgZyHw74YAW9H6hYiYeQ6m0R+GTJsDA= }"
    );
    let out = serde_json::to_string(&ki).unwrap();
    assert_eq!(out.as_str(), raw);
}

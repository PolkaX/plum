// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! A Wrapper of `libp2p_core::PeerId` with the specific CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

pub use libp2p_core::PeerId;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{de, ser, Deserialize, Serialize};

/// A wrapper of `libp2p_core::PeerId` that implement CBOR and JSON serialization/deserialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerIdWrapper(libp2p_core::PeerId);

impl PeerIdWrapper {
    /// Consumes the wrapper, returning the underlying libp2p_core::PeerId.
    pub fn into_inner(self) -> libp2p_core::PeerId {
        self.0
    }

    /// Don't consume the wrapper, borrowing the underlying libp2p_core::PeerId.
    pub fn as_inner(&self) -> &libp2p_core::PeerId {
        &self.0
    }

    /// Don't consume the wrapper, mutable borrowing the underlying libp2p_core::PeerId.
    pub fn as_mut_inner(&mut self) -> &mut libp2p_core::PeerId {
        &mut self.0
    }
}

impl From<libp2p_core::PeerId> for PeerIdWrapper {
    fn from(peer_id: libp2p_core::PeerId) -> Self {
        Self(peer_id)
    }
}

// Implement CBOR serialization for PeerIdWrapper.
impl encode::Encode for PeerIdWrapper {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.str(&self.as_inner().to_string())?.ok()
    }
}

// Implement CBOR deserialization for PeerIdWrapper.
impl<'b> decode::Decode<'b> for PeerIdWrapper {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let peer_id = d
            .str()?
            .parse::<libp2p_core::PeerId>()
            .map_err(|_| decode::Error::Message("Parse PeerId error"))?;
        Ok(PeerIdWrapper(peer_id))
    }
}

// Implement JSON serialization for PeerIdWrapper.
impl ser::Serialize for PeerIdWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::serialize(self.as_inner(), serializer)
    }
}

// Implement JSON deserialization for PeerIdWrapper.
impl<'de> de::Deserialize<'de> for PeerIdWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Self(self::deserialize(deserializer)?))
    }
}

/// A wrapper of `&libp2p_core::PeerId` that implement CBOR and JSON serialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerIdRefWrapper<'a>(&'a libp2p_core::PeerId);

impl<'a> PeerIdRefWrapper<'a> {
    /// Don't consume the wrapper, borrowing the underlying libp2p_core::PeerId.
    pub fn as_inner(&self) -> &libp2p_core::PeerId {
        self.0
    }
}

impl<'a> From<&'a libp2p_core::PeerId> for PeerIdRefWrapper<'a> {
    fn from(peer_id: &'a libp2p_core::PeerId) -> Self {
        Self(peer_id)
    }
}

// Implement CBOR serialization for PeerIdRefWrapper.
impl<'a> encode::Encode for PeerIdRefWrapper<'a> {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.str(&self.as_inner().to_string())?.ok()
    }
}

/// Implement JSON serialization of PeerIdRefWrapper.
impl<'a> ser::Serialize for PeerIdRefWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::serialize(self.as_inner(), serializer)
    }
}

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

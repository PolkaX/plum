// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec};
use serde::{de, ser};

use plum_crypto::{Signature, SignatureType};

use crate::unsigned_message::UnsignedMessage;

/// The signed message (a message with signature).
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct SignedMessage {
    /// The unsigned message.
    pub message: UnsignedMessage,
    /// The signature.
    pub signature: Signature,
}

impl SignedMessage {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        if self.signature.r#type() == SignatureType::Bls {
            return self.message.cid();
        }
        let data = serde_cbor::to_vec(self)
            .expect("CBOR serialization of SignedMessage shouldn't be failed");
        let hash = multihash::Blake2b256::digest(&data);
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    /// Convert to the CID with the given CBOR serialized data of SignedMessage.
    ///
    /// For cases where serialized data of the SignedMessage is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        if self.signature.r#type() == SignatureType::Bls {
            return self.message.cid();
        }
        let hash = multihash::Blake2b256::digest(data.as_ref());
        Cid::new_v1(Codec::DagCBOR, hash)
    }
}

impl ser::Serialize for SignedMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for SignedMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// SignedMessage CBOR serialization/deserialization.
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_crypto::{signature_cbor, Signature};

    use super::SignedMessage;
    use crate::unsigned_message::{cbor as unsigned_message_cbor, UnsignedMessage};

    #[derive(Serialize)]
    struct CborSignedMessageRef<'a>(
        #[serde(with = "unsigned_message_cbor")] &'a UnsignedMessage,
        #[serde(with = "signature_cbor")] &'a Signature,
    );

    /// CBOR serialization.
    pub fn serialize<S>(signed_msg: &SignedMessage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborSignedMessageRef(&signed_msg.message, &signed_msg.signature).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborSignedMessage(
        #[serde(with = "unsigned_message_cbor")] UnsignedMessage,
        #[serde(with = "signature_cbor")] Signature,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SignedMessage, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborSignedMessage(message, signature) = CborSignedMessage::deserialize(deserializer)?;
        Ok(SignedMessage { message, signature })
    }

    /// Vec<SignedMessage> CBOR serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct CborSignedMessageRef<'a>(#[serde(with = "super")] &'a SignedMessage);

        /// CBOR serialization of Vec<SignedMessage>.
        pub fn serialize<S>(signed_msgs: &[SignedMessage], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            signed_msgs
                .iter()
                .map(|signed_msg| CborSignedMessageRef(signed_msg))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct CborSignedMessage(#[serde(with = "super")] SignedMessage);

        /// CBOR deserialization of Vec<SignedMessage>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<SignedMessage>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let signed_msgs = <Vec<CborSignedMessage>>::deserialize(deserializer)?;
            Ok(signed_msgs
                .into_iter()
                .map(|CborSignedMessage(signed_msg)| signed_msg)
                .collect())
        }
    }
}

/// SignedMessage JSON serialization/deserialization.
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_crypto::{signature_json, Signature};

    use super::SignedMessage;
    use crate::unsigned_message::{json as unsigned_message_json, UnsignedMessage};

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonSignedMessageRef<'a> {
        #[serde(with = "unsigned_message_json")]
        message: &'a UnsignedMessage,
        #[serde(with = "signature_json")]
        signature: &'a Signature,
    }

    /// JSON serialization.
    pub fn serialize<S>(signed_msg: &SignedMessage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonSignedMessageRef {
            message: &signed_msg.message,
            signature: &signed_msg.signature,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonSignedMessage {
        #[serde(with = "unsigned_message_json")]
        message: UnsignedMessage,
        #[serde(with = "signature_json")]
        signature: Signature,
    }

    /// JSON deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SignedMessage, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonSignedMessage { message, signature } =
            JsonSignedMessage::deserialize(deserializer)?;
        Ok(SignedMessage { message, signature })
    }

    /// Vec<SignedMessage> JSON serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct JsonSignedMessageRef<'a>(#[serde(with = "super")] &'a SignedMessage);

        /// JSON serialization of Vec<SignedMessage>.
        pub fn serialize<S>(signed_msgs: &[SignedMessage], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            signed_msgs
                .iter()
                .map(|signed_msg| JsonSignedMessageRef(signed_msg))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct JsonSignedMessage(#[serde(with = "super")] SignedMessage);

        /// JSON deserialization of Vec<SignedMessage>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<SignedMessage>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let signed_msgs = <Vec<JsonSignedMessage>>::deserialize(deserializer)?;
            Ok(signed_msgs
                .into_iter()
                .map(|JsonSignedMessage(signed_msg)| signed_msg)
                .collect())
        }
    }
}

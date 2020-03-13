// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use plum_crypto::Signature;

use crate::unsigned_message::UnsignedMessage;

/// The signed message (a message with signature).
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct SignedMessage {
    /// The unsigned message.
    pub message: UnsignedMessage,
    /// The signature.
    pub signature: Signature,
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
    struct TupleSignedMessageRef<'a>(
        #[serde(with = "unsigned_message_cbor")] &'a UnsignedMessage,
        #[serde(with = "signature_cbor")] &'a Signature,
    );

    /// CBOR serialization.
    pub fn serialize<S>(signed_msg: &SignedMessage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let tuple_signed_msg = TupleSignedMessageRef(&signed_msg.message, &signed_msg.signature);
        tuple_signed_msg.serialize(serializer)
    }

    #[derive(Deserialize)]
    struct TupleSignedMessage(
        #[serde(with = "unsigned_message_cbor")] UnsignedMessage,
        #[serde(with = "signature_cbor")] Signature,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SignedMessage, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleSignedMessage(unsigned_message, signature) =
            TupleSignedMessage::deserialize(deserializer)?;
        Ok(SignedMessage {
            message: unsigned_message,
            signature,
        })
    }
}

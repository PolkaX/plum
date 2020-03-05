// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use crate::signature::Signature;

impl ser::Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// Signature CBOR serialization/deserialization.
pub mod cbor {
    use std::convert::TryFrom;

    use serde::{de, ser};
    use serde_bytes::{ByteBuf, Bytes, Deserialize, Serialize};

    use crate::signature::{Signature, SignatureType};

    /// CBOR serialization.
    pub fn serialize<S>(signature: &Signature, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut buf = Vec::with_capacity(1 + signature.bytes.len());
        buf.push(u8::from(signature.ty));
        buf.extend_from_slice(&signature.bytes);
        let value = Bytes::new(&buf);
        value.serialize(serializer)
    }

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let buf = ByteBuf::deserialize(deserializer)?;
        let ty = SignatureType::try_from(buf[0]).map_err(de::Error::custom)?;
        Ok(Signature {
            ty,
            bytes: (&buf[1..]).to_vec(),
        })
    }

    #[test]
    fn signature_cbor_serde() {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct TestSignature(#[serde(with = "self")] Signature);

        let cases = vec![(
            TestSignature(Signature {
                ty: SignatureType::Bls,
                bytes: b"boo! im a signature".to_vec(),
            }),
            vec![
                84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32, 115, 105, 103, 110, 97, 116,
                117, 114, 101,
            ],
        )];

        for (signature, expected) in cases {
            let ser = serde_cbor::to_vec(&signature).unwrap();
            assert_eq!(ser, expected);
            let de = serde_cbor::from_slice::<TestSignature>(&ser).unwrap();
            assert_eq!(signature, de);
        }
    }
}

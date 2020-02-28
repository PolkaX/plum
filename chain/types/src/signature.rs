// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::key_info::SignKeyType;

/// The maximum length of signature.
pub const SIGNATURE_MAX_LENGTH: u32 = 200;

/// The general signature structure.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Signature {
    /// The key type.
    pub ty: SignKeyType,
    /// Tha actual signature data.
    pub data: Vec<u8>,
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = Vec::with_capacity(self.data.len() + 1);
        buf.push(self.ty as u8);
        buf.extend_from_slice(&self.data);
        let data = serde_bytes::Bytes::new(&buf);
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = serde_bytes::ByteBuf::deserialize(deserializer)?;
        let ty = SignKeyType::try_from(buf[0]).map_err(D::Error::custom)?;
        Ok(Self {
            ty,
            data: (&buf[1..]).to_vec(),
        })
    }
}

#[test]
fn signature_serde_should_work() {
    let signature = Signature {
        ty: SignKeyType::BLS,
        data: b"boo! im a signature".to_vec(),
    };
    let expected = [
        84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32, 115, 105, 103, 110, 97, 116, 117, 114,
        101,
    ];
    let ser = serde_cbor::to_vec(&signature).unwrap();
    assert_eq!(ser, &expected[..]);
    let de = serde_cbor::from_slice(&ser).unwrap();
    assert_eq!(signature, de);
}

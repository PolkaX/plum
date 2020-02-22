// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::key_info::KeyType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// The maximum length of signature.
pub const SIGNATURE_MAX_LENGTH: u32 = 200;

/// The general signature structure.
// #[derive(Eq, PartialEq, Debug, Clone, Serialize_tuple, Deserialize_tuple)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Signature {
    /// The key type.
    pub ty: KeyType,
    /// Tha actual signature data.
    // #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = serde_bytes::Bytes::new(&self.data);
        let ty_clone = vec![self.ty.clone() as u8];
        let ty = serde_bytes::Bytes::new(&ty_clone);
        let to_ser = (ty, data);
        to_ser.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let out: (KeyType, serde_bytes::ByteBuf) = Deserialize::deserialize(deserializer)?;
        Ok(Self {
            ty: out.0,
            data: out.1.into_vec(),
        })
    }
}

#[test]
fn signature_serde_should_work() {
    let signature = Signature {
        ty: KeyType::BLS,
        data: b"boo! im a signature".to_vec(),
    };
    let expected = [
        84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32, 115, 105, 103, 110, 97, 116, 117, 114,
        101,
    ];

    let key_ty = KeyType::BLS;
    println!("--- key_ty: {:?}", serde_cbor::to_vec(&key_ty).unwrap());

    let ser = serde_cbor::to_vec(&signature).unwrap();
    assert_eq!(ser, &expected[..]);
    let de = serde_cbor::from_slice(&ser).unwrap();
    assert_eq!(signature, de);
}

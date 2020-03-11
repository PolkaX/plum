use serde::{de::Error, Deserializer, Serializer};

pub mod h256 {
    use super::*;
    use primitive_types::H256;
    pub mod raw {
        use super::*;
        pub fn serialize<S>(bytes: &H256, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serde_bytes::serialize(bytes.as_ref(), serializer)
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<H256, D::Error>
        where
            D: Deserializer<'de>,
        {
            let buf: Vec<u8> = serde_bytes::deserialize(deserializer)?;
            if buf.len() != H256::len_bytes() {
                return Err(D::Error::custom("H256 length must be 32 Bytes"));
            }
            Ok(H256::from_slice(buf.as_slice()))
        }
    }

    pub mod option {
        use super::*;
        pub fn serialize<S>(bytes: &Option<H256>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let b: &[u8] = bytes.as_ref().map(AsRef::as_ref).unwrap_or_default();
            serde_bytes::serialize(b, serializer)
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<H256>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let buf: Vec<u8> = serde_bytes::deserialize(deserializer)?;
            let len = buf.len();
            if len == 0 {
                Ok(None)
            } else {
                if len != H256::len_bytes() {
                    Err(D::Error::custom("H256 length must be 32 Bytes"))
                } else {
                    Ok(Some(H256::from_slice(buf.as_slice())))
                }
            }
        }
    }
}

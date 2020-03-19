// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt::{self, Display};

use cid::Cid;

/// A TipsetKey is an immutable collection of CIDs forming a unique key for a tipset.
// The CIDs are assumed to be distinct and in canonical order. Two keys with the same
// CIDs in a different order are not considered equal.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct TipsetKey {
    cids: Vec<Cid>,
}

impl TipsetKey {
    /// Create a new TipsetKey with the given collection of CIDs.
    pub fn new(cids: Vec<Cid>) -> Self {
        Self { cids }
    }

    /// Return the inner CIDs.
    pub fn cids(&self) -> &[Cid] {
        &self.cids
    }

    /// Returns `true` if the key contains no CIDs.
    pub fn is_empty(&self) -> bool {
        self.cids.is_empty()
    }
}

impl Display for TipsetKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cids = self
            .cids
            .iter()
            .map(|cid| cid.to_string())
            .collect::<Vec<_>>();
        write!(f, "{:?}", cids)
    }
}

/// TipsetKey CBOR serialization/deserialization, need to use `serde_cbor::Serializer` and `serde_cbor::Deserializer`
pub mod cbor {
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

    use super::TipsetKey;

    #[derive(Serialize)]
    struct CborCidRef<'a>(#[serde(with = "cid::ipld_dag_cbor")] &'a Cid);

    /// CBOR serialization.
    pub fn serialize<S>(key: &TipsetKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let cids = key
            .cids
            .iter()
            .map(|cid| CborCidRef(cid))
            .collect::<Vec<_>>();
        cids.serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborCid(#[serde(with = "cid::ipld_dag_cbor")] Cid);

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<TipsetKey, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let cids = Vec::<CborCid>::deserialize(deserializer)?;
        Ok(TipsetKey {
            cids: cids.into_iter().map(|cid| cid.0).collect(),
        })
    }
}

/// TipsetKey JSON serialization/deserialization
pub mod json {
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

    use super::TipsetKey;

    #[derive(Serialize)]
    struct JsonCidRef<'a>(#[serde(with = "cid::ipld_dag_json")] &'a Cid);

    /// JSON serialization.
    pub fn serialize<S>(key: &TipsetKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let cids = key
            .cids
            .iter()
            .map(|cid| JsonCidRef(cid))
            .collect::<Vec<_>>();
        cids.serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonCid(#[serde(with = "cid::ipld_dag_json")] Cid);

    /// JSON deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<TipsetKey, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let cids = Vec::<JsonCid>::deserialize(deserializer)?;
        Ok(TipsetKey {
            cids: cids.into_iter().map(|cid| cid.0).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use cid::{Cid, Codec};
    use multihash::Blake2b256;
    use serde::{Deserialize, Serialize};

    use super::TipsetKey;

    fn cid1() -> Cid {
        Cid::new_v1(Codec::DagCBOR, Blake2b256::digest(b"a"))
    }
    fn cid2() -> Cid {
        Cid::new_v1(Codec::DagCBOR, Blake2b256::digest(b"b"))
    }
    fn cid3() -> Cid {
        Cid::new_v1(Codec::DagCBOR, Blake2b256::digest(b"c"))
    }

    #[test]
    fn equality() {
        assert_eq!(TipsetKey::new(vec![cid1()]), TipsetKey::new(vec![cid1()]));
        assert_eq!(
            TipsetKey::new(vec![cid1(), cid2(), cid3()]),
            TipsetKey::new(vec![cid1(), cid2(), cid3()])
        );
        assert_ne!(TipsetKey::new(vec![cid1()]), TipsetKey::new(vec![cid2()]));
        assert_ne!(
            TipsetKey::new(vec![cid1(), cid3(), cid2()]),
            TipsetKey::new(vec![cid1(), cid2(), cid3()])
        );
    }

    #[test]
    fn tipset_key_cbor_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CborTipsetKey(#[serde(with = "super::cbor")] TipsetKey);

        #[rustfmt::skip]
        let cases  = vec![
            (vec![], "80"),
            (
                vec![cid1()],
                "81\
                d82a5827000171a0e402208928aae63c84d87ea098564d1e03ad813f107add474e56aedd286349c0c03ea4"
            ),
            (
                vec![cid1(), cid2(), cid3()],
                "83\
                d82a5827000171a0e402208928aae63c84d87ea098564d1e03ad813f107add474e56aedd286349c0c03ea4\
                d82a5827000171a0e402206e5c1f45cbaf19f94230ba3501c378a5335af71a331b5b5aed62792332288dc3\
                d82a5827000171a0e40220ed5402299a6208014e0f5f25ae6ca3badddc95db67dce164cb8aa086bd48978a"
            ),
        ];

        for (cids, expected) in cases {
            let key = CborTipsetKey(TipsetKey::new(cids));
            let ser = serde_cbor::to_vec(&key).unwrap();
            let hex = hex::encode(ser);
            assert_eq!(hex, expected);

            let bytes = hex::decode(expected).unwrap();
            let de = serde_cbor::from_slice::<CborTipsetKey>(&bytes).unwrap();
            assert_eq!(de, key);
        }
    }

    #[test]
    fn tipset_key_json_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct JsonTipsetKey(#[serde(with = "super::json")] TipsetKey);

        let cases = vec![
            (vec![], "[]"),
            (
                vec![cid1()],
                "[{\"/\":\"bafy2bzacecesrkxghscnq7vatble2hqdvwat6ed23vdu4vvo3uuggsoaya7ki\"}]",
            ),
            (
                vec![cid1(), cid2(), cid3()],
                "[\
                    {\"/\":\"bafy2bzacecesrkxghscnq7vatble2hqdvwat6ed23vdu4vvo3uuggsoaya7ki\"},\
                    {\"/\":\"bafy2bzacebxfyh2fzoxrt6kcgc5dkaodpcstgwxxdizrww225vrhsizsfcg4g\"},\
                    {\"/\":\"bafy2bzacedwviarjtjraqakob5pslltmuo5n3xev3nt5zylezofkbbv5jclyu\"}\
                ]",
            ),
        ];

        for (cids, expected) in cases {
            let key = JsonTipsetKey(TipsetKey::new(cids));
            let ser = serde_json::to_string(&key).unwrap();
            assert_eq!(ser, expected);
            let de = serde_json::from_str::<JsonTipsetKey>(expected).unwrap();
            assert_eq!(de, key);
        }
    }
}

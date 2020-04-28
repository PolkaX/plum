// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

///
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct BeaconEntry {
    round: u64,
    data: Vec<u8>,
    prev_round: u64,
}

impl BeaconEntry {
    /// Create a new BeachEntry with given round, prev round and data.
    pub fn new(round: u64, prev_round: u64, data: Vec<u8>) -> Self {
        Self {
            round,
            data,
            prev_round,
        }
    }

    /// Get previous round.
    pub fn prev_round(&self) -> u64 {
        self.prev_round
    }
}

impl ser::Serialize for BeaconEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for BeaconEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// BeaconEntry CBOR serialization/deserialization
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use super::BeaconEntry;

    #[derive(Serialize)]
    struct CborBeaconEntryRef<'a>(&'a u64, #[serde(with = "serde_bytes")] &'a Vec<u8>, &'a u64);

    /// CBOR serialization
    pub fn serialize<S>(beacon_entry: &BeaconEntry, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborBeaconEntryRef(
            &beacon_entry.round,
            &beacon_entry.data,
            &beacon_entry.prev_round,
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborBeaconEntry(u64, #[serde(with = "serde_bytes")] Vec<u8>, u64);

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BeaconEntry, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborBeaconEntry(round, data, prev_round) = CborBeaconEntry::deserialize(deserializer)?;
        Ok(BeaconEntry {
            round,
            data,
            prev_round,
        })
    }

    /// Vec<BeaconEntry> CBOR serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct CborBeaconEntryRef<'a>(#[serde(with = "super")] &'a BeaconEntry);

        /// CBOR serialization of Vec<BeaconEntry>.
        pub fn serialize<S>(
            beacon_entries: &[BeaconEntry],
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            beacon_entries
                .iter()
                .map(|beacon_entry| CborBeaconEntryRef(beacon_entry))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct CborBeaconEntry(#[serde(with = "super")] BeaconEntry);

        /// CBOR deserialization of Vec<BeaconEntry>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BeaconEntry>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let beacon_entries = <Vec<CborBeaconEntry>>::deserialize(deserializer)?;
            Ok(beacon_entries
                .into_iter()
                .map(|CborBeaconEntry(beacon_entry)| beacon_entry)
                .collect())
        }
    }
}

/// BeaconEntry JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::BeaconEntry;

    #[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct JsonBeaconEntryRef<'a> {
        round: &'a u64,
        #[serde(with = "plum_types::base64")]
        data: &'a [u8],
        prev_round: &'a u64,
    }

    /// JSON serialization
    pub fn serialize<S>(beacon_entry: &BeaconEntry, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonBeaconEntryRef {
            round: &beacon_entry.round,
            data: &beacon_entry.data,
            prev_round: &beacon_entry.prev_round,
        }
        .serialize(serializer)
    }

    #[derive(Eq, PartialEq, Debug, Clone, Hash, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct JsonBeaconEntry {
        round: u64,
        #[serde(with = "plum_types::base64")]
        data: Vec<u8>,
        prev_round: u64,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BeaconEntry, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonBeaconEntry {
            round,
            data,
            prev_round,
        } = JsonBeaconEntry::deserialize(deserializer)?;
        Ok(BeaconEntry {
            round,
            data,
            prev_round,
        })
    }

    /// Vec<BeaconEntry> JSON serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct JsonBeaconEntryRef<'a>(#[serde(with = "super")] &'a BeaconEntry);

        /// JSON serialization of Vec<BeaconEntry>.
        pub fn serialize<S>(
            beacon_entries: &[BeaconEntry],
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            beacon_entries
                .iter()
                .map(|beacon_entry| JsonBeaconEntryRef(beacon_entry))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct JsonBeaconEntry(#[serde(with = "super")] BeaconEntry);

        /// JSON deserialization of Vec<BeaconEntry>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BeaconEntry>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let beacon_entries = <Vec<JsonBeaconEntry>>::deserialize(deserializer)?;
            Ok(beacon_entries
                .into_iter()
                .map(|JsonBeaconEntry(beacon_entry)| beacon_entry)
                .collect())
        }
    }
}

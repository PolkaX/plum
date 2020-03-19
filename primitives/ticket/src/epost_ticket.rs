// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

/// PoSt election candidates
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct EPostTicket {
    ///
    pub partial: Vec<u8>,
    ///
    pub sector_id: u64,
    ///
    pub challenge_index: u64,
}

impl EPostTicket {
    /// Create a new EPostTicket with the given `partial`, `sector_id` and `challenge_index`.
    pub fn new<T: Into<Vec<u8>>>(partial: T, sector_id: u64, challenge_index: u64) -> Self {
        Self {
            partial: partial.into(),
            sector_id,
            challenge_index,
        }
    }
}

impl ser::Serialize for EPostTicket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for EPostTicket {
    fn deserialize<D>(deserializer: D) -> Result<EPostTicket, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// EPostTicket CBOR serialization/deserialization
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use super::EPostTicket;

    #[derive(Serialize)]
    struct TupleEPostTicketRef<'a>(#[serde(with = "serde_bytes")] &'a [u8], &'a u64, &'a u64);

    /// CBOR serialization
    pub fn serialize<S>(epost_ticket: &EPostTicket, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        TupleEPostTicketRef(
            &epost_ticket.partial,
            &epost_ticket.sector_id,
            &epost_ticket.challenge_index,
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct TupleEPostTicket(#[serde(with = "serde_bytes")] Vec<u8>, u64, u64);

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<EPostTicket, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleEPostTicket(partial, sector_id, challenge_index) =
            TupleEPostTicket::deserialize(deserializer)?;
        Ok(EPostTicket {
            partial,
            sector_id,
            challenge_index,
        })
    }

    #[test]
    fn epost_ticket_cbor_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CborEPostTicket(#[serde(with = "self")] EPostTicket);

        let cases = vec![(
            CborEPostTicket(EPostTicket {
                partial: b"epost_ticket".to_vec(),
                sector_id: 6,
                challenge_index: 8,
            }),
            vec![
                131, 76, 101, 112, 111, 115, 116, 95, 116, 105, 99, 107, 101, 116, 6, 8,
            ],
        )];

        for (epost_ticket, expected) in cases {
            let ser = serde_cbor::to_vec(&epost_ticket).unwrap();
            assert_eq!(ser, expected);
            let de = serde_cbor::from_slice::<CborEPostTicket>(&ser).unwrap();
            assert_eq!(de, epost_ticket);
        }
    }
}

/// EPostTicket JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use super::EPostTicket;

    #[derive(Serialize, Deserialize)]
    struct JsonEPostTicket {
        #[serde(rename = "Partial")]
        partial: String,
        #[serde(rename = "SectorID")]
        sector_id: u64,
        #[serde(rename = "ChallengeIndex")]
        challenge_index: u64,
    }

    /// JSON serialization
    pub fn serialize<S>(epost_ticket: &EPostTicket, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonEPostTicket {
            partial: base64::encode(&epost_ticket.partial),
            sector_id: epost_ticket.sector_id,
            challenge_index: epost_ticket.challenge_index,
        }
        .serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<EPostTicket, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let epost_ticket = JsonEPostTicket::deserialize(deserializer)?;
        Ok(EPostTicket {
            partial: base64::decode(epost_ticket.partial).expect("base64 decode shouldn't be fail"),
            sector_id: epost_ticket.sector_id,
            challenge_index: epost_ticket.challenge_index,
        })
    }

    #[test]
    fn epost_ticket_json_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct JsonEPostTicket(#[serde(with = "self")] EPostTicket);

        let cases = vec![(
            JsonEPostTicket(EPostTicket {
                partial: b"epost_ticket".to_vec(),
                sector_id: 6,
                challenge_index: 8,
            }),
            r#"{"Partial":"ZXBvc3RfdGlja2V0","SectorID":6,"ChallengeIndex":8}"#,
        )];

        for (epost_ticket, expected) in cases {
            let ser = serde_json::to_string(&epost_ticket).unwrap();
            assert_eq!(ser, expected);
            let de = serde_json::from_str::<JsonEPostTicket>(&ser).unwrap();
            assert_eq!(de, epost_ticket);
        }
    }
}

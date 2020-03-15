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
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::EPostTicket;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CborEPostTicket(#[serde(with = "super::cbor")] EPostTicket);

    #[test]
    fn epost_ticket_cbor_serde() {
        let epost_ticket = CborEPostTicket(EPostTicket {
            partial: b"epost_ticket".to_vec(),
            sector_id: 6,
            challenge_index: 8,
        });
        let expected = [
            131, 76, 101, 112, 111, 115, 116, 95, 116, 105, 99, 107, 101, 116, 6, 8,
        ];

        let ser = serde_cbor::to_vec(&epost_ticket).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice::<CborEPostTicket>(&ser).unwrap();
        assert_eq!(de, epost_ticket);
    }
}

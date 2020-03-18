// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! Ticket, EPostTicket and EPostProof with CBOR and JSON serialization/deserialization

#![deny(missing_docs)]

mod epost_proof;
mod epost_ticket;
mod ticket;

pub use self::epost_proof::{cbor as epost_proof_cbor, json as epost_proof_json, EPostProof};
pub use self::epost_ticket::{cbor as epost_ticket_cbor, json as epost_ticket_json, EPostTicket};
pub use self::ticket::{cbor as ticket_cbor, json as ticket_json, Ticket};

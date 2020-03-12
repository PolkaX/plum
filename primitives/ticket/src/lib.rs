// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod epost_proof;
mod epost_ticket;
mod ticket;

pub use self::epost_proof::EPostProof;
pub use self::epost_ticket::{cbor as epost_ticket_cbor, EPostTicket};
pub use self::ticket::{cbor as ticket_cbor, Ticket};

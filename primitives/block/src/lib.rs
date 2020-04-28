// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! BeaconEntry, BlockHeader, Block, MsgMeta, Ticket and ElectionProof
//! with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod beacon_entry;
mod block;
mod block_msg;
mod election_proof;
mod header;
mod msg_meta;
// TODO: need to move the module to abi
mod post_proof;
mod ticket;

pub use self::beacon_entry::{cbor as beacon_entry_cbor, json as beacon_entry_json, BeaconEntry};
pub use self::block::{cbor as block_cbor, json as block_json, Block};
pub use self::block_msg::{cbor as block_msg_cbor, json as block_msg_json, BlockMsg};
pub use self::election_proof::{
    cbor as election_proof_cbor, json as election_proof_json, ElectionProof,
};
pub use self::header::{cbor as block_header_cbor, json as block_header_json, BlockHeader};
pub use self::msg_meta::{cbor as msg_meta_cbor, json as msg_meta_json, MsgMeta};
pub use self::ticket::{cbor as ticket_cbor, json as ticket_json, Ticket};

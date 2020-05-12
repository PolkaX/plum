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

pub use self::beacon_entry::BeaconEntry;
pub use self::block::Block;
pub use self::block_msg::BlockMsg;
pub use self::election_proof::ElectionProof;
pub use self::header::BlockHeader;
pub use self::msg_meta::MsgMeta;
pub use self::ticket::Ticket;

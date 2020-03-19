// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! BlockHeader, Block, and MsgMeta with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod block;
mod block_msg;
mod header;
mod msg_meta;

pub use self::block::{cbor as block_cbor, Block};
pub use self::block_msg::{cbor as block_msg_cbor, BlockMsg};
pub use self::header::{cbor as block_header_cbor, json as block_header_json, BlockHeader};
pub use self::msg_meta::{cbor as msg_meta_cbor, MsgMeta};

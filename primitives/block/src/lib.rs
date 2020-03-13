// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! BlockHeader, Block, and MsgMeta with CBOR serialization/deserialization.

#![deny(missing_docs)]

mod block;
mod header;
mod msg_meta;

pub use self::block::{cbor as block_cbor, Block};
pub use self::header::{cbor as block_header_cbor, BlockHeader};
pub use self::msg_meta::{cbor as msg_meta_cbor, MsgMeta};

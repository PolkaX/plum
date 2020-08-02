// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use super::proto;
pub use super::proto::message::BlockPresenceType;

/// A struct represents HAVE / DONT_HAVE for a given Cid.
#[derive(Clone, PartialEq)]
pub struct BlockPresence {
    cid: Cid,
    r#type: BlockPresenceType, // have: 0, don't have: 1
}

impl From<&BlockPresence> for proto::message::BlockPresence {
    fn from(block_presence: &BlockPresence) -> Self {
        Self {
            cid: block_presence.cid.to_bytes(),
            r#type: block_presence.r#type as i32,
        }
    }
}

impl BlockPresence {
    /// Create a new block presence struct with the give cid and type.
    pub fn new(cid: Cid, r#type: BlockPresenceType) -> Self {
        Self { cid, r#type }
    }

    /// Return the size of the block presence struct in the protobuf.
    pub fn size(&self) -> usize {
        prost::Message::encoded_len(&proto::message::BlockPresence::from(self))
    }
}

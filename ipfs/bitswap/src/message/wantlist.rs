// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;
use std::collections::HashMap;

use cid::Cid;

use super::proto;
pub use super::proto::message::wantlist::WantType;

/// A raw list of wanted blocks and their priorities.
pub type WantList = HashMap<Cid, WantListEntry>;

/// Priority of a want list entry.
pub type Priority = i32;

/// A entry in want list, consisting of a cid and its priority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WantListEntry {
    pub(crate) cid: Cid,
    pub(crate) priority: Priority,
    pub(crate) want_type: WantType, // block: 0, have: 1
    /// indicate whether message is a cancel.
    pub cancel: bool,
    /// indicate whether requester wants a DONT_HAVE message.
    /// indicate whether requester wants a HAVE message (instead of the block)
    pub send_dont_have: bool,
}

impl PartialOrd for WantListEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WantListEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority.cmp(&other.priority) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl From<&WantListEntry> for proto::message::wantlist::Entry {
    fn from(entry: &WantListEntry) -> Self {
        Self {
            block: entry.cid.to_bytes(),
            priority: entry.priority,
            cancel: entry.cancel,
            want_type: entry.want_type as i32,
            send_dont_have: entry.send_dont_have,
        }
    }
}

impl WantListEntry {
    /// Create a new want list entry.
    pub fn new(cid: Cid, priority: Priority) -> Self {
        Self {
            cid,
            priority,
            want_type: WantType::Block,
            cancel: false,
            send_dont_have: false,
        }
    }

    /// Return the size of the entry in the protobuf.
    pub fn size(&self) -> usize {
        prost::Message::encoded_len(&proto::message::wantlist::Entry::from(self))
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod block;
mod block_presence;
mod prefix;
/// The generated types from `message.proto`.
#[allow(missing_docs)]
pub mod proto;
mod wantlist;

pub use self::block::Block;
pub use self::block_presence::{BlockPresence, BlockPresenceType};
pub use self::prefix::Prefix;
pub use self::wantlist::{Priority, WantList, WantListEntry, WantType};

use std::collections::HashMap;

use cid::Cid;

use ipfs_block::IpfsBlock;

/// A bitswap message.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BitswapMessage {
    /// Whether it is the full list of wanted blocks.
    full: bool,
    /// List of wanted blocks.
    wantlist: WantList,
    /// List of blocks to send.
    blocks: HashMap<Cid, IpfsBlock>,
    /// List of HAVE / DONT_HAVE in the message
    block_presences: HashMap<Cid, BlockPresenceType>, // have: 0, don't have: 1
    /// Number of outstanding bytes of data that the engine has yet to send to the client.
    pending_bytes: i32,
}

impl BitswapMessage {
    /// Create a new, empty bitswap message.
    pub fn new(full: bool) -> Self {
        Self {
            full,
            wantlist: WantList::new(),
            blocks: HashMap::new(),
            block_presences: HashMap::new(),
            pending_bytes: 0,
        }
    }

    /// Reset the values in the message back to defaults, so it can be reused.
    pub fn reset(&mut self, full: bool) {
        self.full = full;
        self.wantlist.clear();
        self.blocks.clear();
        self.block_presences.clear();
        self.pending_bytes = 0;
    }

    /// Indicates whether the message has any information.
    pub fn is_empty(&self) -> bool {
        self.wantlist.len() == 0 && self.blocks.len() == 0 && self.block_presences.len() == 0
    }

    /// Return the size of the message in bytes.
    // FIXME: ???
    pub fn size(&self) -> usize {
        let mut size = 0usize;
        size += self
            .wantlist
            .values()
            .map(|entry| entry.size())
            .sum::<usize>();
        size += self
            .blocks
            .values()
            .map(|block| block.data().len())
            .sum::<usize>();
        size += self
            .block_presences
            .iter()
            .map(|(cid, r#type)| BlockPresence::new(cid.clone(), *r#type).size())
            .sum::<usize>();
        size
    }

    /// Check if the want list is full.
    /// A full wantlist is an authoritative copy, a 'non-full' wantlist is a patch-set.
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Return a slice of unique keys that represent data wanted by the sender.
    pub fn wantlist(&self) -> &WantList {
        &self.wantlist
    }

    /// Add an entry to the Wantlist.
    /// Return the size of the entry in the protobuf if the entry is not exist in the message.
    pub fn add_entry(
        &mut self,
        cid: Cid,
        priority: Priority,
        want_type: WantType,
        send_dont_have: bool,
    ) -> Option<usize> {
        self._add_entry(cid, priority, want_type, false, send_dont_have)
    }

    /// Add a CANCEL for the given CID to the message.
    /// Return the size of the CANCEL entry in the protobuf if the entry is not exist in the message.
    pub fn cancel(&mut self, cid: Cid) -> Option<usize> {
        self._add_entry(cid, 0, WantType::Block, true, false)
    }

    fn _add_entry(
        &mut self,
        cid: Cid,
        priority: Priority,
        want_type: WantType,
        cancel: bool,
        send_dont_have: bool,
    ) -> Option<usize> {
        if let Some(entry) = self.wantlist.get_mut(&cid) {
            // Only change priority if want is of the same type
            if entry.want_type == want_type {
                entry.priority = priority;
            }
            // Only change from "dont cancel" to "do cancel"
            if cancel {
                entry.cancel = cancel;
            }
            // Only change from "dont send" to "do send" DONT_HAVE
            if send_dont_have {
                entry.send_dont_have = send_dont_have;
            }
            // want-block overrides existing want-have
            if want_type == WantType::Block && entry.want_type == WantType::Have {
                entry.want_type = want_type;
            }
            None
        } else {
            let entry = WantListEntry {
                cid: cid.clone(),
                priority,
                want_type,
                cancel,
                send_dont_have,
            };
            let size = entry.size();
            self.wantlist.insert(cid, entry);
            Some(size)
        }
    }

    /// Remove any entries for the given CID.
    /// Useful when the want status for the CID changes when preparing a message.
    pub fn remove(&mut self, cid: &Cid) -> Option<WantListEntry> {
        self.wantlist.remove(cid)
    }

    /// Return a slice of unique blocks.
    pub fn blocks(&self) -> Vec<IpfsBlock> {
        self.blocks.values().cloned().collect::<Vec<_>>()
    }

    /// Add a block to the message.
    pub fn add_block(&mut self, block: IpfsBlock) {
        self.block_presences.remove(block.cid());
        self.blocks.insert(block.cid().clone(), block);
    }

    /// Return the list of HAVE / DONT_HAVE in the message.
    pub fn block_presences(&self) -> Vec<BlockPresence> {
        self.block_presences
            .iter()
            .map(|(cid, r#type)| BlockPresence::new(cid.clone(), *r#type))
            .collect::<Vec<_>>()
    }

    /// Add a HAVE / DONT_HAVE for the given Cid to the message.
    pub fn add_block_presence(&mut self, cid: Cid, r#type: BlockPresenceType) {
        if let Some(_) = self.blocks.get(&cid) {
            return;
        }
        self.block_presences.insert(cid, r#type);
    }

    /// Return the CIDs for each HAVE.
    pub fn haves(&self) -> Vec<Cid> {
        self.block_presences
            .iter()
            .filter_map(|(cid, bpt)| match bpt {
                BlockPresenceType::Have => Some(cid),
                BlockPresenceType::DontHave => None,
            })
            .cloned()
            .collect()
    }

    /// Add a HAVE for the given Cid to the message.
    pub fn add_have(&mut self, cid: Cid) {
        self.add_block_presence(cid, BlockPresenceType::Have);
    }

    /// Return the CIDs for each DONT_HAVE.
    pub fn dont_haves(&self) -> Vec<Cid> {
        self.block_presences
            .iter()
            .filter_map(|(cid, bpt)| match bpt {
                BlockPresenceType::Have => None,
                BlockPresenceType::DontHave => Some(cid),
            })
            .cloned()
            .collect()
    }

    /// Add a DONT_HAVE for the given Cid to the message.
    pub fn add_dont_have(&mut self, cid: Cid) {
        self.add_block_presence(cid, BlockPresenceType::DontHave);
    }

    /// Return the number of outstanding bytes of data that the engine has yet to send to the client.
    pub fn pending_bytes(&self) -> i32 {
        self.pending_bytes
    }

    /// Set the number of outstanding bytes of data that the engine has yet to send to the client.
    pub fn set_pending_bytes(&mut self, pending_bytes: i32) {
        self.pending_bytes = pending_bytes;
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::{HashMap, HashSet};

use cid::Cid;
use prost::Message;

use ipfs_block::IpfsBlock;

use crate::proto;

/// Priority of a wanted block.
pub type Priority = i32;

/// A bitswap message.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BitswapMessage {
    /// List of wanted blocks.
    want: HashMap<Cid, Priority>,
    /// List of blocks to cancel.
    cancel: HashSet<Cid>,
    /// Whether it is the full list of wanted blocks.
    full: bool,
    /// List of blocks to send.
    blocks: Vec<IpfsBlock>,
}

impl BitswapMessage {
    /// Check whether the queued message is empty.
    pub fn is_empty(&self) -> bool {
        self.want.is_empty() && self.cancel.is_empty() && self.blocks.is_empty()
    }

    /// Return the list of blocks.
    pub fn blocks(&self) -> &[IpfsBlock] {
        &self.blocks
    }

    /// Return the list of wanted blocks.
    pub fn want(&self) -> &HashMap<Cid, Priority> {
        &self.want
    }

    /// Return the list of cancelled blocks.
    pub fn cancel(&self) -> &HashSet<Cid> {
        &self.cancel
    }

    /// Add a `Block` to the message.
    pub fn add_block(&mut self, block: IpfsBlock) {
        self.blocks.push(block);
    }

    /// Remove the block from the message.
    pub fn remove_block(&mut self, cid: &Cid) {
        self.blocks.retain(|block| block.cid() != cid);
    }

    /// Add a block to the want list.
    pub fn add_want_block(&mut self, cid: &Cid, priority: Priority) {
        self.want.insert(cid.to_owned(), priority);
    }

    /// Removes the block from the want list.
    pub fn remove_want_block(&mut self, cid: &Cid) {
        self.want.remove(cid);
    }

    /// Add a block to the cancel list.
    pub fn cancel_block(&mut self, cid: &Cid) {
        self.cancel.insert(cid.to_owned());
    }
}

impl BitswapMessage {
    /// Encode the `message` into bytes that can be sent to a substream.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut proto: proto::Message = proto::Message::default();

        let mut wantlist: proto::message::Wantlist = proto::message::Wantlist::default();


        if wantlist.entries.is_empty() {
            proto.wantlist = Some(wantlist);
        }

        let mut bytes = Vec::with_capacity(proto.encoded_len());
        proto
            .encode(&mut bytes)
            .expect("protobuf message should be valid");
        bytes
    }

    /// Decode a `message` from bytes that were received from a substream.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        let proto = proto::Message::decode(bytes)?;
        let mut message = Self::default();

        Ok(message)
    }
}

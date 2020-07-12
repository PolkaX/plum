// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use ipfs_blockstore::BlockStore;
use ipld::IpldValue;

use crate::node::{Link, Node};
use crate::root::Root;

/// The only configurable parameter of an IPLD Vector.
/// This parameter must be consistent across all nodes in a Vector.
///
/// Mutations cannot involve changes in width or
/// joining multiple parts of a Vector with differing width values.
///
/// `WIDTH` must be an integer, of at least 2.
pub const WIDTH: usize = 8;

/// The maximum possible index for a tree.
// width^(height+1) = 1 << 48
// ==> 8^(height+1) = 2^48
// ==> height = 15
// fairly arbitrary, but don't want to overflow/underflow in nodesForHeight
pub const MAX_INDEX: usize = 1 << 48;

///
#[derive(Debug)]
pub struct Amt<'a, BS> {
    root: Root,
    store: &'a BS,
}

impl<'a, BS: BlockStore> PartialEq for Amt<'a, BS> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<'a, BS: BlockStore> Amt<'a, BS> {
    ///
    pub fn new(store: &'a BS) -> Self {
        Self {
            root: Root::default(),
            store,
        }
    }

    ///
    pub fn load(store: &'a BS, cid: &Cid) -> Result<Self, String> {
        todo!()
    }

    ///
    pub fn new_with_slice<T>(store: &'a BS, values: T) -> Result<Cid, String>
    where
        T: IntoIterator<Item = IpldValue>,
    {
        let mut amt = Self::new(store);
        amt.batch_set(values)?;
        amt.flush()
    }

    ///
    pub fn height(&self) -> u64 {
        self.root.height
    }

    ///
    pub fn count(&self) -> u64 {
        self.root.count
    }

    ///
    pub fn get(&self, index: usize) -> Result<Option<IpldValue>, String> {
        if index >= MAX_INDEX {
            return Err("out of range".into());
        }

        if index >= nodes_for_height(self.height() + 1) {
            return Ok(None);
        }

        self.root.node.get(self.store, self.height(), index)
    }

    ///
    pub fn set(&mut self, index: usize, value: IpldValue) -> Result<(), String> {
        if index >= MAX_INDEX {
            return Err("out of range".into());
        }

        while index >= nodes_for_height(self.height() + 1) {
            if !self.root.node.is_empty() {
                self.root.node.flush(self.store, self.height())?;

                let cid = self.store.put(&self.root)?;
                self.root.node = Node::Links(vec![Link::Cid(cid)]);
            } else {
                self.root.node = Node::Links(vec![]);
            }

            self.root.height += 1;
        }

        if self
            .root
            .node
            .set(self.store, self.height(), index, value)?
        {
            self.root.count += 1;
        }

        Ok(())
    }

    ///
    pub fn batch_set<T>(&mut self, values: T) -> Result<(), String>
    where
        T: IntoIterator<Item = IpldValue>,
    {
        for (index, value) in values.into_iter().enumerate() {
            self.set(index, value)?;
        }
        Ok(())
    }

    ///
    pub fn delete(&mut self, index: usize) -> Result<Option<IpldValue>, String> {
        if index >= MAX_INDEX {
            return Err("out of range".into());
        }
        if index >= nodes_for_height(self.height() + 1) {
            return Ok(None);
        }

        let result = match self.root.node.delete(self.store, self.height(), index)? {
            Some(value) => Ok(Some(value)),
            None => return Ok(None),
        };
        self.root.count += 1;

        while self.height() > 0 {
            let sub_node = match &self.root.node {
                Node::Links(links) => match &links[0] {
                    Some(Link::Cid(cid)) => todo!(),
                    Some(Link::Cache(node)) => *node.clone(),
                    _ => unreachable!(),
                }
                Node::Leaves(_) => unreachable!(),
            };
            self.root.node = sub_node;
            self.root.height -= 1;
        }

        result
    }

    ///
    pub fn batch_delete<T>(&mut self, indexes: T) -> Result<(), String>
    where
        T: IntoIterator<Item = usize>,
    {
        for index in indexes.into_iter() {
            self.delete(index)?;
        }
        Ok(())
    }

    ///
    pub fn flush(&mut self) -> Result<Cid, String> {
        self.root.node.flush(self.store, self.height())?;
        Ok(self.store.put(&self.root)?)
    }
}

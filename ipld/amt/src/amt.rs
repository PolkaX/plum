// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::{anyhow, Result};
use cid::Cid;

use ipld::{IpldStore, IpldValue};

use crate::error::IpldAmtError;
use crate::node::{Link, Node};
use crate::root::Root;
use crate::nodes_for_height;

/// The maximum possible index for a tree.
// width^(height+1) = 1 << 48
// ==> 8^(height+1) = 2^48
// ==> height = 15
// fairly arbitrary, but don't want to overflow/underflow in nodesForHeight
pub const MAX_INDEX: usize = 1 << 48;

///
#[derive(Debug)]
pub struct Amt<'a, S> {
    root: Root,
    store: &'a mut S,
}

impl<'a, S: IpldStore> PartialEq for Amt<'a, S> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<'a, S: IpldStore> Amt<'a, S> {
    ///
    pub fn new(store: &'a mut S) -> Self {
        Self {
            root: Root::default(),
            store,
        }
    }

    ///
    pub fn load(store: &'a mut S, cid: &Cid) -> Result<Self> {
        let root = IpldStore::get(store, cid)?.ok_or_else(|| IpldAmtError::NotFound)?;
        Ok(Self { root, store })
    }

    ///
    pub fn new_with_slice<T>(store: &'a mut S, values: T) -> Result<Cid>
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
    pub fn get(&self, index: usize) -> Result<Option<&IpldValue>> {
        if index >= MAX_INDEX {
            return Err(anyhow!("out of range"));
        }

        if index >= nodes_for_height(self.height() + 1) {
            return Ok(None);
        }

        self.root.node.get(self.store, self.height(), index)
    }

    ///
    pub fn set(&mut self, index: usize, value: IpldValue) -> Result<()> {
        if index >= MAX_INDEX {
            return Err(anyhow!("out of range"));
        }

        while index >= nodes_for_height(self.height() + 1) {
            if !self.root.node.is_empty() {
                self.root.node.flush(self.store, self.height())?;

                let cid = IpldStore::put(self.store, &self.root)?;
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
    pub fn batch_set<T>(&mut self, values: T) -> Result<()>
    where
        T: IntoIterator<Item = IpldValue>,
    {
        for (index, value) in values.into_iter().enumerate() {
            self.set(index, value)?;
        }
        Ok(())
    }

    ///
    pub fn delete(&mut self, index: usize) -> Result<Option<IpldValue>> {
        if index >= MAX_INDEX {
            return Err(anyhow!("out of range"));
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
                    Link::Cid(cid) => todo!(),
                    Link::Cache(node) => *node.clone(),
                },
                Node::Leaves(_) => unreachable!(),
            };
            self.root.node = sub_node;
            self.root.height -= 1;
        }

        result
    }

    ///
    pub fn batch_delete<T>(&mut self, indexes: T) -> Result<()>
    where
        T: IntoIterator<Item = usize>,
    {
        for index in indexes.into_iter() {
            self.delete(index)?;
        }
        Ok(())
    }

    ///
    pub fn flush(&mut self) -> Result<Cid> {
        self.root.node.flush(self.store, self.height())?;
        Ok(IpldStore::put(self.store, &self.root)?)
    }
}

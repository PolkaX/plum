// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::{anyhow, Result};
use cid::Cid;

use ipld::{IpldStore, IpldValue};

use crate::error::IpldAmtError;
use crate::max_leaf_value_size_for;
use crate::node::{Link, Node};
use crate::root::Root;

/// The maximum possible index for a AMT.
// width^(height+1) = 1 << 48
// ==> 8^(height+1) = 2^48
// ==> height = 15
// fairly arbitrary, but don't want to overflow/underflow in nodesForHeight
pub const MAX_INDEX: usize = 1 << 48;

/// The IPLD AMT (Array Mapped Tries).
#[derive(Debug)]
pub struct Amt<S> {
    root: Root,
    store: S,
}

impl<S: IpldStore> PartialEq for Amt<S> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<S: IpldStore> Amt<S> {
    /// Create a new IPLD AMT (Array Mapped Tries) root node with a block store.
    pub fn new(store: S) -> Self {
        Self {
            root: Root::default(),
            store,
        }
    }

    /// Load a new AMT with a block store and a cid of the root of the AMT.
    pub fn load(store: S, cid: &Cid) -> Result<Self> {
        let root = IpldStoreget(&store, cid)?.ok_or_else(|| IpldAmtError::NotFound)?;
        Ok(Self { root, store })
    }

    /// Create a new AMT with a block store and set a slice of IPLD values into the block store,
    /// return the cid of the root of the AMT.
    pub fn new_with_slice<T>(store: S, values: T) -> Result<Cid>
    where
        T: IntoIterator<Item = IpldValue>,
    {
        let mut amt = Self::new(store);
        amt.batch_set(values)?;
        amt.flush()
    }

    /// Return the height of the root node of the AMT.
    pub fn height(&self) -> u64 {
        self.root.height
    }

    /// Return the count of the nodes of the AMT.
    pub fn count(&self) -> u64 {
        self.root.count
    }

    /// Get the IPLD value at the index of the AMT.
    pub fn get(&self, index: usize) -> Result<Option<&IpldValue>> {
        if index >= MAX_INDEX {
            return Err(anyhow!("out of range"));
        }

        if index >= max_leaf_value_size_for(self.height()) {
            return Ok(None);
        }

        self.root.node.get(&self.store, self.height(), index)
    }

    /// Set the IPLD value at the index of the AMT.
    pub fn set(&mut self, index: usize, value: IpldValue) -> Result<()> {
        if index >= MAX_INDEX {
            return Err(anyhow!("out of range"));
        }

        while index >= max_leaf_value_size_for(self.height()) {
            if !self.root.node.is_empty() {
                self.root.node.flush(&mut self.store, self.root.height)?;
                let cid = IpldStore::put(&mut self.store, &self.root)?;
                self.root.node = Node::Links(vec![Link::Cid(cid)]);
            } else {
                self.root.node = Node::Links(vec![]);
            }

            self.root.height += 1;
        }

        if self
            .root
            .node
            .set(&mut self.store, self.root.height, index, value)?
        {
            self.root.count += 1;
        }

        Ok(())
    }

    /// Batch set the IPLD values into the AMT.
    pub fn batch_set<T>(&mut self, values: T) -> Result<()>
    where
        T: IntoIterator<Item = IpldValue>,
    {
        for (index, value) in values.into_iter().enumerate() {
            self.set(index, value)?;
        }
        Ok(())
    }

    /// Delete the IPLD value at the index of the AMT.
    pub fn delete(&mut self, index: usize) -> Result<Option<IpldValue>> {
        if index >= MAX_INDEX {
            return Err(anyhow!("out of range"));
        }
        if index >= max_leaf_value_size_for(self.height()) {
            return Ok(None);
        }

        let result = match self
            .root
            .node
            .delete(&mut self.store, self.root.height, index)?
        {
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

    /// Batch delete the IPLD values from the AMT.
    pub fn batch_delete<T>(&mut self, indexes: T) -> Result<()>
    where
        T: IntoIterator<Item = usize>,
    {
        for index in indexes.into_iter() {
            self.delete(index)?;
        }
        Ok(())
    }

    /// Flush the root node into the block store and return the cid of the root.
    pub fn flush(&mut self) -> Result<Cid> {
        self.root.node.flush(&mut self.store, self.root.height)?;
        Ok(IpldStore::put(&mut self.store, &self.root)?)
    }
}

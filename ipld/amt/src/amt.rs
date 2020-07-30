// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use ipld::{IpldStore, IpldValue};

use crate::bitmap::BitMap;
use crate::error::{IpldAmtError, Result};
use crate::node::{Link, Node};
use crate::root::Root;
use crate::{max_leaf_value_size_for, WIDTH};

/// The maximum possible index for a AMT.
// width^(height+1) = 1 << 48
// ==> 8^(height+1) = 2^48
// ==> height = 15
// fairly arbitrary, but don't want to overflow/underflow in nodesForHeight
pub const MAX_INDEX: usize = 1 << 48;

/// The IPLD AMT (Array Mapped Tries).
#[derive(Debug)]
pub struct IpldAmt<S> {
    root: Root,
    store: S,
}

impl<S: IpldStore> PartialEq for IpldAmt<S> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<S: IpldStore> IpldAmt<S> {
    /// Create a new IPLD AMT (Array Mapped Tries) root node with a block store.
    pub fn new(store: S) -> Self {
        Self {
            root: Root::default(),
            store,
        }
    }

    /// Load a new AMT with a block store and a cid of the root of the AMT.
    pub fn load(store: S, cid: &Cid) -> Result<Self> {
        let root = IpldStore::get::<Root>(&store, cid)?.ok_or_else(|| IpldAmtError::CidNotFound)?;
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
    pub fn get(&self, index: usize) -> Result<Option<IpldValue>> {
        if index >= MAX_INDEX {
            return Err(IpldAmtError::IndexOutOfRange(index));
        }
        if index >= max_leaf_value_size_for(self.height()) {
            return Ok(None);
        }

        self.root.node.get(&self.store, self.height(), index)
    }

    /// Set the IPLD value at the index of the AMT.
    pub fn set(&mut self, index: usize, value: IpldValue) -> Result<()> {
        if index >= MAX_INDEX {
            return Err(IpldAmtError::IndexOutOfRange(index));
        }

        while index >= max_leaf_value_size_for(self.height()) {
            if !self.root.node.is_empty() {
                self.root.node.flush(&mut self.store, self.root.height)?;

                let cid = IpldStore::put(&mut self.store, &self.root)?;
                let mut links = <[Option<Link>; WIDTH]>::default();
                links[0] = Some(Link::Cid(cid));

                self.root.node = Node::Link {
                    bitmap: BitMap(0x01),
                    links,
                };
            } else {
                self.root.node = Node::Link {
                    bitmap: BitMap::default(),
                    links: Default::default(),
                }
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
        // TODO: there are more optimized ways of doing this method
        for (index, value) in values.into_iter().enumerate() {
            self.set(index, value)?;
        }
        Ok(())
    }

    /// Delete the IPLD value at the index of the AMT.
    pub fn delete(&mut self, index: usize) -> Result<Option<IpldValue>> {
        if index >= MAX_INDEX {
            return Err(IpldAmtError::IndexOutOfRange(index));
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
        self.root.count -= 1;

        // handle height changes.
        while self.root.node.bitmap() == 0x01 && self.height() > 0 {
            let sub_node = match &self.root.node {
                Node::Link { links, .. } => match &links[0] {
                    Some(link) => link.load_node(&self.store)?,
                    None => unreachable!("The link node with non-zero must exist"),
                },
                Node::Leaf { .. } => {
                    unreachable!("The node with non-zero height can not be a leaf node")
                }
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
        // TODO: there's a faster way of doing this, but this works for now
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

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use ipfs_blockstore::BlockStore;
use ipld::{cbor_from_ipld, cbor_to_ipld};

use crate::error::HamtError;
use crate::node::Node;

// const ARRAY_WIDTH: usize = 3;
const DEFAULT_BIT_WIDTH: usize = 8;

/// Implementation of the HAMT data structure for IPLD.
pub struct Hamt<'a, BS: BlockStore> {
    root: Node,
    bit_width: usize,
    store: &'a BS,
}

impl<'a, BS: BlockStore> PartialEq for Hamt<'a, BS> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<'a, BS: BlockStore> Hamt<'a, BS> {
    /// Create a new HAMT with the given store and default bit width.
    pub fn new(store: &'a BS) -> Self {
        Self::new_with_bit_width(store, DEFAULT_BIT_WIDTH)
    }

    /// Create a new HAMT with given store and bit width.
    pub fn new_with_bit_width(store: &'a BS, bit_width: usize) -> Self {
        Self {
            root: Node::default(),
            bit_width,
            store,
        }
    }

    ///
    pub fn load(store: &'a BS, cid: &Cid) -> Result<Self, HamtError> {
        Self::load_with_bit_width(store, cid, DEFAULT_BIT_WIDTH)
    }

    ///
    pub fn load_with_bit_width(
        store: &'a BS,
        cid: &Cid,
        bit_width: usize,
    ) -> Result<Self, HamtError> {
        match store.get::<Node>(cid)? {
            Some(root) => Ok(Self {
                root,
                bit_width,
                store,
            }),
            None => Err(HamtError::CidNotFound(cid.to_string())),
        }
    }

    ///
    pub fn root(&self) -> &Node {
        &self.root
    }

    ///
    pub fn bit_width(&self) -> usize {
        self.bit_width
    }

    ///
    pub fn store(&self) -> &'a BS {
        self.store
    }

    ///
    pub fn set<V>(&mut self, key: &str, value: V) -> Result<(), HamtError>
    where
        V: minicbor::Encode,
    {
        let value = cbor_to_ipld(&value)?;
        self.root.set(key, value, self.store, self.bit_width)
    }

    ///
    pub fn get<V>(&self, key: &str) -> Result<Option<V>, HamtError>
    where
        V: for<'b> minicbor::Decode<'b>,
    {
        match self.root.get(key, self.store, self.bit_width)? {
            Some(value) => Ok(Some(cbor_from_ipld(&value)?)),
            None => Ok(None),
        }
    }

    ///
    pub fn remove(&mut self, key: &str) -> Result<bool, HamtError> {
        match self.root.remove(key, self.store, self.bit_width)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    ///
    pub fn flush(&mut self) -> Result<(), HamtError> {
        self.root.flush(self.store)
    }

    ///
    pub fn for_each<F, V>(&self, mut f: F) -> Result<(), HamtError>
    where
        F: FnMut(&str, V) -> Result<(), HamtError>,
        V: for<'b> minicbor::Decode<'b>,
    {
        self.root.for_each(self.store, &mut f)
    }
}

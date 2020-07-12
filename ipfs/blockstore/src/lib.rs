// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The BlockStore interface.

#![deny(missing_docs)]

use anyhow::Result;
use cid::Cid;

use ipfs_block::Block;

/// BlockStore wraps a DataStore block-centered methods and provides a layer
/// of abstraction which allows to add different caching strategies.
pub trait BlockStore {
    /// Delete the block for given `cid`.
    /// If the `cid` is not in the block store, this method returns no error.
    fn delete_block(&mut self, cid: &Cid) -> Result<()>;

    /// Return whether the `cid` is mapped to a `block`.
    fn has(&self, cid: &Cid) -> Result<bool>;

    /// Retrieve the `block` named by `cid`.
    fn get(&self, cid: &Cid) -> Result<Option<Box<dyn Block>>>;

    /// Return the CIDs mapped BlockSize.
    fn get_size(&self, cid: &Cid) -> Result<usize>;

    /// Put a given block to the underlying datastore
    fn put<B: Block>(&mut self, block: B) -> Result<()>;

    /// Puts a slice of blocks at the same time using batching capabilities of
    /// the underlying datastore whenever possible.
    fn put_many<B: Block>(&mut self, blocks: &[B]) -> Result<()>;

    /// Specifies if every read block should be rehashed to make sure it matches its CID.
    fn hash_on_read(&mut self, enabled: bool);
}

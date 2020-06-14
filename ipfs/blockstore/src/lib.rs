// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The BlockStore interface.

#![deny(missing_docs)]

use cid::Cid;

use plum_ipfs_block::Block;

/// The error type used for block store.
#[doc(hidden)]
#[derive(Clone, Debug, thiserror::Error)]
pub enum BlockStoreError {
    #[error("block '{0}' not found")]
    NotFound(Cid),
    #[error("{0}")]
    Custom(String),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for BlockStoreError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        BlockStoreError::Custom(err.to_string())
    }
}

/// BlockStore wraps a DataStore block-centered methods and provides a layer
/// of abstraction which allows to add different caching strategies.
pub trait BlockStore {
    /// Delete the block for given `cid`.
    /// If the `cid` is not in the block store, this method returns no error.
    fn delete_block(&self, cid: &Cid) -> Result<(), BlockStoreError>;

    /// Return whether the `cid` is mapped to a `block`.
    fn has(&self, cid: &Cid) -> Result<bool, BlockStoreError>;

    /// Retrieve the `block` named by `cid`.
    fn get<B: Block>(&self, cid: &Cid) -> Result<B, BlockStoreError>;

    /// Return the CIDs mapped BlockSize.
    fn get_size(&self, cid: &Cid) -> Result<usize, BlockStoreError>;

    /// Put a given block to the underlying datastore
    fn put<B: Block>(&mut self, block: B) -> Result<(), BlockStoreError>;

    /// Puts a slice of blocks at the same time using batching capabilities of
    /// the underlying datastore whenever possible.
    fn put_many<B: Block>(&mut self, blocks: &[B]) -> Result<(), BlockStoreError>;

    /// Specifies if every read block should be rehashed to make sure it matches its CID.
    fn hash_on_read(&mut self, enable: bool);
}

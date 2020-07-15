// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The BlockStore interface.

#![deny(missing_docs)]

use std::io::Result;

use cid::Cid;

use ipfs_block::{Block, IpfsBlock};
use ipfs_datastore::{DataStoreBatch, DataStoreRead, DataStoreWrite, Key, ToBatchDataStore};

/// BlockStore wraps a DataStore block-centered methods and provides a layer
/// of abstraction which allows to add different caching strategies.
pub trait BlockStore: ToBatchDataStore {
    /// Return whether the `cid` is mapped to a `block`.
    fn has(&self, cid: &Cid) -> Result<bool> {
        let key = multihash_to_datastore_key(cid.hash().as_bytes());
        <Self as DataStoreRead>::has(self, &key)
    }

    /// Retrieve the `block` named by `cid`.
    fn get(&self, cid: &Cid) -> Result<Option<Box<dyn Block>>> {
        let key = multihash_to_datastore_key(cid.hash().as_bytes());
        match <Self as DataStoreRead>::get(self, &key)? {
            Some(data) => Ok(Some(Box::new(unsafe {
                IpfsBlock::new_unchecked(data, cid.clone())
            }))),
            None => Ok(None),
        }
    }

    /// Put a given block to the underlying datastore
    fn put<B: Block>(&mut self, block: B) -> Result<()> {
        let key = multihash_to_datastore_key(block.cid().hash().as_bytes());
        if <Self as DataStoreRead>::has(self, &key)? {
            Ok(()) // already store
        } else {
            <Self as DataStoreWrite>::put(self, key, block.data().to_vec())
        }
    }

    /// Puts a slice of blocks at the same time using batching capabilities of
    /// the underlying datastore whenever possible.
    fn put_many<B: Block>(&mut self, blocks: &[B]) -> Result<()> {
        let mut batch = self.batch()?;
        for block in blocks {
            let key = multihash_to_datastore_key(block.cid().hash().as_bytes());
            if <Self as DataStoreRead>::has(self, &key)? {
                continue;
            } else {
                batch.put(key, block.data().to_vec())?;
            }
        }
        batch.commit()
    }

    /// Delete the block for given `cid`.
    /// If the `cid` is not in the block store, this method returns no error.
    fn delete(&mut self, cid: &Cid) -> Result<()> {
        let key = multihash_to_datastore_key(cid.hash().as_bytes());
        <Self as DataStoreWrite>::delete(self, &key)
    }

    /*
    /// Specifies if every read block should be rehashed to make sure it matches its CID.
    fn hash_on_read(&mut self, enabled: bool);
    */
}

impl<T: ToBatchDataStore> BlockStore for T {}

// Create a Key from the given multihash.
// If working with Cids, you can call cid.hash() to obtain the multihash.
// Note that different CIDs might represent the same multihash.
fn multihash_to_datastore_key<T: AsRef<[u8]>>(mh: T) -> Key {
    let base = multibase::Base::Base32Upper.encode(mh);
    unsafe { Key::new_unchecked(format!("/{}", base)) }
}

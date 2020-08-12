// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use ipfs_block::Block;
use ipfs_blockstore::BlockStore;

use crate::error::IpldError;

/// IpldStore wraps block store and provides an interface for storing and retrieving CBOR encoded data.
pub trait IpldStore: BlockStore {
    /// Get an object from the block store by the cid.
    fn get<T>(&self, cid: &Cid) -> Result<Option<T>, IpldError>
    where
        T: for<'b> minicbor::Decode<'b>,
    {
        match <Self as BlockStore>::get(self, cid)? {
            Some(block) => {
                let data = block.data();
                Ok(Some(minicbor::decode(data)?))
            }
            None => Ok(None),
        }
    }

    /// Put an object into the block store.
    fn put<T>(&mut self, value: T) -> Result<Cid, IpldError>
    where
        T: minicbor::Encode,
    {
        let block = Block::new(value);
        let cid = block.cid().clone();
        <Self as BlockStore>::put(self, block)?;
        Ok(cid)
    }
}

impl<T: BlockStore> IpldStore for T {}

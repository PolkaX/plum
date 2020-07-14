// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::Result;
use cid::Cid;

use ipfs_block::IpfsBlock;
use ipfs_blockstore::BlockStore;

/// IpldStore wraps block store and provides an interface for storing and retrieving CBOR encoded data.
pub trait IpldStore: BlockStore {
    /// Get an object from the block store by the cid.
    fn get<T>(&self, cid: &Cid) -> Result<Option<T>>
    where
        T: for<'b> minicbor::Decode<'b>,
    {
        match BlockStore::get(self, cid)? {
            Some(block) => {
                let data = (*block).data();
                Ok(Some(minicbor::decode(data)?))
            }
            None => Ok(None),
        }
    }

    /// Put an object into the block store.
    fn put<T>(&mut self, value: T) -> Result<Cid>
    where
        T: minicbor::Encode,
    {
        let block = IpfsBlock::new(value);
        let cid = block.cid().clone();
        BlockStore::put(self, block)?;
        Ok(cid)
    }
}

impl<T: BlockStore> IpldStore for T {}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use ipfs_blockstore::BlockStore;
use ipld::IpldValue;

use crate::root::Root;

///
#[derive(Debug)]
pub struct Amt<'a, BS> {
    root: Root,
    store: &'a BS,
}

impl<'a, BS> Amt<'a, BS>
where
    BS: BlockStore,
{
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
        todo!()
    }

    ///
    pub fn set(&mut self, index: usize, value: IpldValue) -> Result<(), String> {
        todo!()
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
    pub fn delete(&mut self, index: usize) -> Result<(), String> {
        todo!()
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
        todo!()
    }
}

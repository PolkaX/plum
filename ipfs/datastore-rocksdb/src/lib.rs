// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

use std::borrow::Borrow;

use ipfs_datastore::{DataStore, DataStoreError, DataStoreRead, DataStoreWrite, Key};

pub(crate) type Result<T> = std::result::Result<T, DataStoreError>;

///
pub struct RocksDbDataStore {}

impl DataStore for RocksDbDataStore {
    fn sync<K>(&mut self, _prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        unimplemented!()
    }

    fn close(&mut self) -> Result<()> {
        unimplemented!()
    }
}

impl DataStoreRead for RocksDbDataStore {
    fn get<K>(&self, _key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        unimplemented!()
    }

    fn has<K>(&self, _key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        unimplemented!()
    }

    fn size<K>(&self, _key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        unimplemented!()
    }
}

impl DataStoreWrite for RocksDbDataStore {
    fn put<K, V>(&mut self, _key: K, _value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        unimplemented!()
    }

    fn delete<K>(&mut self, _key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        unimplemented!()
    }
}

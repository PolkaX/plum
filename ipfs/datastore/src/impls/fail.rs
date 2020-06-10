// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use crate::error::Result;
use crate::key::Key;
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};
use crate::store::{Persistent, PersistentDataStore};

/// The user-provided fail function.
pub type FailFunc = Box<dyn Fn(&str) -> Result<()>>;

/// FailDataStore is a datastore which fails according to a user-provided function.
pub struct FailDataStore<DS: DataStore> {
    err_func: FailFunc,
    child: DS,
}

impl<DS: DataStore> FailDataStore<DS> {
    /// Create a new datastore with the given error function.
    /// The `err_func` will be called with different strings depending on the datastore function.
    pub fn new(err_func: FailFunc, datastore: DS) -> Self {
        Self {
            err_func,
            child: datastore,
        }
    }
}

impl<DS: DataStore> DataStore for FailDataStore<DS> {
    fn sync<K>(&self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        (self.err_func)("sync")?;
        self.child.sync(prefix)
    }

    fn close(&self) -> Result<()> {
        self.child.close()
    }
}

impl<DS: DataStore> DataStoreRead for FailDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        (self.err_func)("put")?;
        self.child.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        (self.err_func)("has")?;
        self.child.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        (self.err_func)("size")?;
        self.child.size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for FailDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        (self.err_func)("put")?;
        self.child.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        (self.err_func)("delete")?;
        self.child.delete(key)
    }
}

impl<DS: PersistentDataStore> Persistent for FailDataStore<DS> {
    fn disk_usage(&self) -> Result<u64> {
        (self.err_func)("disk-usage")?;
        self.child.disk_usage()
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use crate::error::Result;
use crate::key::Key;
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};
use crate::store::{Persistent, PersistentDataStore};

/// The delay interface for delay operation.
pub trait Delay {
    /// Wait for a period of time duration util time out before applying the operation.
    fn wait(&self);
}

/// DelayDataStore is an adapter that delays operations on the inner datastore.
pub struct DelayDataStore<DL: Delay, DS: DataStore> {
    delay: DL,
    child: DS,
}

impl<DL: Delay, DS: DataStore> DelayDataStore<DL, DS> {
    /// Create a new DelayDataStore.
    pub fn new(delay: DL, datastore: DS) -> Self {
        Self {
            delay,
            child: datastore,
        }
    }
}

impl<DL: Delay, DS: DataStore> DataStore for DelayDataStore<DL, DS> {
    fn sync<K>(&self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        self.delay.wait();
        self.child.sync(prefix)
    }

    fn close(&self) -> Result<()> {
        self.child.close()
    }
}

impl<DL: Delay, DS: DataStore> DataStoreRead for DelayDataStore<DL, DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.child.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.child.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.child.size(key)
    }
}

impl<DL: Delay, DS: DataStore> DataStoreWrite for DelayDataStore<DL, DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.delay.wait();
        self.child.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.delay.wait();
        self.child.delete(key)
    }
}

impl<DL: Delay, DS: PersistentDataStore> Persistent for DelayDataStore<DL, DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.delay.wait();
        self.child.disk_usage()
    }
}

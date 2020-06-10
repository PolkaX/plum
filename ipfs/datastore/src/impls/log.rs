// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use log::info;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Check, CheckedDataStore};
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};
use crate::store::{Gc, GcDataStore};
use crate::store::{Persistent, PersistentDataStore};
use crate::store::{Scrub, ScrubbedDataStore};

/// LogDataStore logs all accesses through the datastore.
#[derive(Clone, Debug)]
pub struct LogDataStore<DS: DataStore> {
    name: String,
    child: DS,
}

impl<DS: DataStore> LogDataStore<DS> {
    /// Create a new LogDataStore.
    pub fn new<S: Into<String>>(name: S, datastore: DS) -> Self {
        Self {
            name: name.into(),
            child: datastore,
        }
    }
}

impl<DS: DataStore> DataStore for LogDataStore<DS> {
    fn sync<K>(&self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        let prefix = prefix.into();
        info!("{}: sync {}", self.name, prefix);
        self.child.sync(prefix)
    }

    fn close(&self) -> Result<()> {
        info!("{}: close", self.name);
        self.child.close()
    }
}

impl<DS: DataStore> DataStoreRead for LogDataStore<DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        info!("{}: get {}", self.name, key.borrow());
        self.child.get(key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        info!("{}: has {}", self.name, key.borrow());
        self.child.has(key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        info!("{}: size {}", self.name, key.borrow());
        self.child.size(key)
    }
}

impl<DS: DataStore> DataStoreWrite for LogDataStore<DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = key.into();
        let value = value.into();
        info!("{}: put {} - {:?}", self.name, key, value);
        self.child.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        info!("{}: delete {}", self.name, key.borrow());
        self.child.delete(key)
    }
}

impl<DS: CheckedDataStore> Check for LogDataStore<DS> {
    fn check(&self) -> Result<()> {
        info!("{}: check", self.name);
        self.child.check()
    }
}

impl<DS: GcDataStore> Gc for LogDataStore<DS> {
    fn collect_garbage(&self) -> Result<()> {
        info!("{}: collect_garbage", self.name);
        self.child.collect_garbage()
    }
}

impl<DS: PersistentDataStore> Persistent for LogDataStore<DS> {
    fn disk_usage(&self) -> Result<u64> {
        info!("{}: disk_usage", self.name);
        self.child.disk_usage()
    }
}

impl<DS: ScrubbedDataStore> Scrub for LogDataStore<DS> {
    fn scrub(&self) -> Result<()> {
        info!("{}: scrub", self.name);
        self.child.scrub()
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow::Borrow;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Check, CheckedDataStore};
use crate::store::{DataStore, DataStoreRead, DataStoreWrite};
use crate::store::{Gc, GcDataStore};
use crate::store::{Persistent, PersistentDataStore};
use crate::store::{Scrub, ScrubbedDataStore};

/// KeyTransform is an object with a pair of functions for transforming keys invertibly.
pub trait KeyTransform {
    ///
    fn convert_key<K>(&self, key: &K) -> Key
    where
        K: Borrow<Key>;

    ///
    fn invert_key<K>(&self, key: &K) -> Key
    where
        K: Borrow<Key>;
}

///
pub struct TransformDataStore<KT: KeyTransform, DS: DataStore> {
    transform: KT,
    child: DS,
}

impl<KT: KeyTransform, DS: DataStore> TransformDataStore<KT, DS> {
    ///
    pub fn new(transform: KT, datastore: DS) -> Self {
        Self {
            transform,
            child: datastore,
        }
    }
}

impl<KT: KeyTransform, DS: DataStore> DataStore for TransformDataStore<KT, DS> {
    fn sync<K>(&self, prefix: K) -> Result<()>
    where
        K: Into<Key>,
    {
        self.child.sync(self.transform.convert_key(&prefix.into()))
    }

    fn close(&self) -> Result<()> {
        self.child.close()
    }
}

impl<KT: KeyTransform, DS: DataStore> DataStoreRead for TransformDataStore<KT, DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        self.child.get(&self.transform.convert_key(key))
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        self.child.has(&self.transform.convert_key(key))
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        self.child.size(&self.transform.convert_key(key))
    }
}

impl<KT: KeyTransform, DS: DataStore> DataStoreWrite for TransformDataStore<KT, DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        self.child
            .put(self.transform.convert_key(&key.into()), value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        self.child.delete(&self.transform.convert_key(key))
    }
}

impl<KT: KeyTransform, DS: CheckedDataStore> Check for TransformDataStore<KT, DS> {
    fn check(&self) -> Result<()> {
        self.child.check()
    }
}

impl<KT: KeyTransform, DS: GcDataStore> Gc for TransformDataStore<KT, DS> {
    fn collect_garbage(&self) -> Result<()> {
        self.child.collect_garbage()
    }
}

impl<KT: KeyTransform, DS: PersistentDataStore> Persistent for TransformDataStore<KT, DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.child.disk_usage()
    }
}

impl<KT: KeyTransform, DS: ScrubbedDataStore> Scrub for TransformDataStore<KT, DS> {
    fn scrub(&self) -> Result<()> {
        self.child.scrub()
    }
}

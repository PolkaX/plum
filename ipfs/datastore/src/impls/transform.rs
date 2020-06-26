// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.Batching

use std::borrow::Borrow;

use crate::error::Result;
use crate::key::Key;
use crate::store::{Batch, BatchDataStore};
use crate::store::{Check, CheckedDataStore};
use crate::store::{DataStore, DataStoreBatch, DataStoreRead, DataStoreWrite};
use crate::store::{Gc, GcDataStore};
use crate::store::{Persistent, PersistentDataStore};
use crate::store::{Scrub, ScrubbedDataStore};

/// KeyTransform is an data store with a pair of functions for transforming keys invertibly.
pub trait KeyTransform: Clone {
    /// Convert `origin` key into `target` key.
    fn convert_key<K: Borrow<Key>>(&self, key: &K) -> Key;

    /// Invert `target` key into `origin` key
    fn invert_key<K: Borrow<Key>>(&self, key: &K) -> Key;
}

/// TransformDataStore is a datastore with a pair of KeyTransform functions.
#[derive(Clone)]
pub struct TransformDataStore<KT: KeyTransform, DS: DataStore> {
    transform: KT,
    datastore: DS,
}

impl<KT: KeyTransform, DS: DataStore> TransformDataStore<KT, DS> {
    /// Create a new TransformDataStore.
    pub fn new(transform: KT, datastore: DS) -> Self {
        Self {
            transform,
            datastore,
        }
    }
}

impl<KT: KeyTransform, DS: DataStore> DataStore for TransformDataStore<KT, DS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(prefix);
        self.datastore.sync(&key)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl<KT: KeyTransform, DS: DataStore> DataStoreRead for TransformDataStore<KT, DS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.get(&key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.has(&key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.size(&key)
    }
}

impl<KT: KeyTransform, DS: DataStore> DataStoreWrite for TransformDataStore<KT, DS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = self.transform.convert_key(&key.into());
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.delete(&key)
    }
}

impl<KT: KeyTransform, DS: CheckedDataStore> Check for TransformDataStore<KT, DS> {
    fn check(&self) -> Result<()> {
        self.datastore.check()
    }
}

impl<KT: KeyTransform, DS: GcDataStore> Gc for TransformDataStore<KT, DS> {
    fn collect_garbage(&self) -> Result<()> {
        self.datastore.collect_garbage()
    }
}

impl<KT: KeyTransform, DS: PersistentDataStore> Persistent for TransformDataStore<KT, DS> {
    fn disk_usage(&self) -> Result<u64> {
        self.datastore.disk_usage()
    }
}

impl<KT: KeyTransform, DS: ScrubbedDataStore> Scrub for TransformDataStore<KT, DS> {
    fn scrub(&self) -> Result<()> {
        self.datastore.scrub()
    }
}

impl<KT: KeyTransform, BDS: BatchDataStore> Batch for TransformDataStore<KT, BDS> {
    type Batch = TransformBatchDataStore<KT, BDS>;

    fn batch(&self) -> Result<Self::Batch> {
        Ok(TransformBatchDataStore::new(
            self.transform.clone(),
            self.datastore.clone(),
        ))
    }
}

// ============================================================================

/// TransformBatchDataStore is a batching datastore with a KeyMap function.
#[derive(Clone)]
pub struct TransformBatchDataStore<KT: KeyTransform, BDS: BatchDataStore> {
    transform: KT,
    datastore: BDS,
}

impl<KT: KeyTransform, BDS: BatchDataStore> TransformBatchDataStore<KT, BDS> {
    /// Create a new TransformBatchDataStore.
    pub fn new(transform: KT, datastore: BDS) -> Self {
        Self {
            transform,
            datastore,
        }
    }
}

impl<KT: KeyTransform, BDS: BatchDataStore> DataStore for TransformBatchDataStore<KT, BDS> {
    fn sync<K>(&mut self, prefix: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(prefix);
        self.datastore.sync(&key)
    }

    fn close(&mut self) -> Result<()> {
        self.datastore.close()
    }
}

impl<KT: KeyTransform, BDS: BatchDataStore> DataStoreRead for TransformBatchDataStore<KT, BDS> {
    fn get<K>(&self, key: &K) -> Result<Vec<u8>>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.get(&key)
    }

    fn has<K>(&self, key: &K) -> Result<bool>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.has(&key)
    }

    fn size<K>(&self, key: &K) -> Result<usize>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.size(&key)
    }
}

impl<KT: KeyTransform, BDS: BatchDataStore> DataStoreWrite for TransformBatchDataStore<KT, BDS> {
    fn put<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: Into<Key>,
        V: Into<Vec<u8>>,
    {
        let key = self.transform.convert_key(&key.into());
        self.datastore.put(key, value)
    }

    fn delete<K>(&mut self, key: &K) -> Result<()>
    where
        K: Borrow<Key>,
    {
        let key = self.transform.convert_key(key);
        self.datastore.delete(&key)
    }
}

impl<KT: KeyTransform, BDS: BatchDataStore> DataStoreBatch for TransformBatchDataStore<KT, BDS> {
    fn commit(&mut self) -> Result<()> {
        self.datastore.commit()
    }
}

// ============================================================================

/// KeyMapFn is a function that maps one key to another.
pub trait KeyMapFn: Clone + Fn(&Key) -> Key {}

//// KeyTransformPair is a convince struct for constructing a key transform.
#[doc(hidden)]
#[derive(Clone)]
pub struct KeyTransformPair<C: KeyMapFn, I: KeyMapFn> {
    pub convert: C,
    pub invert: I,
}

impl<C: KeyMapFn, I: KeyMapFn> KeyTransform for KeyTransformPair<C, I> {
    fn convert_key<K: Borrow<Key>>(&self, key: &K) -> Key {
        (self.convert)(key.borrow())
    }

    fn invert_key<K: Borrow<Key>>(&self, key: &K) -> Key {
        (self.invert)(key.borrow())
    }
}

///  PrefixTransform constructs a KeyTransform with a pair of functions that
///  add or remove the given prefix key.
///
/// # Panics
///
/// Inverting key will panic if prefix not found when it should be there.
#[doc(hidden)]
#[derive(Clone)]
pub struct PrefixTransform {
    pub prefix: Key,
}

impl KeyTransform for PrefixTransform {
    fn convert_key<K: Borrow<Key>>(&self, key: &K) -> Key {
        self.prefix.child(key.borrow())
    }

    fn invert_key<K: Borrow<Key>>(&self, key: &K) -> Key {
        let key = key.borrow();
        if self.prefix.is_root() {
            return key.to_owned();
        }

        if self.prefix.is_ancestor_of(key) {
            let prefix_len = self.prefix.as_str().len();
            unsafe { Key::new_unchecked(&key.as_str()[prefix_len..]) }
        } else {
            panic!("expected prefix not found");
        }
    }
}

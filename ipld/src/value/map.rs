// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

/// The Map kind of IPLD Data Model.
pub type Map<K, V> = BTreeMap<K, V>;

/// In DAG-CBOR, map keys must be strings, as defined by the [IPLD Data Model](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md)
/// The keys in every map must be sorted lowest value to highest.
/// Sorting is performed on the bytes of the representation of the keys.
/// - If two keys have different lengths, the shorter one sorts earlier;
/// - If two keys have the same length, the one with the lower value in (byte-wise) lexical order sorts earlier.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapKey(String);

impl MapKey {
    /// Convert self into inner string.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Extracts a string slice containing the entire `String`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Ord for MapKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.len() != other.0.len() {
            // If two keys have different lengths, the shorter one sorts earlier
            self.0.len().cmp(&other.0.len())
        } else {
            // If two keys have the same length, the one with the lower value in (byte-wise) lexical order sorts earlier.
            self.0.cmp(&other.0)
        }
    }
}

impl PartialOrd for MapKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for MapKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for MapKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl borrow::Borrow<str> for MapKey {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl borrow::BorrowMut<str> for MapKey {
    fn borrow_mut(&mut self) -> &mut str {
        self.0.borrow_mut()
    }
}

impl From<String> for MapKey {
    fn from(s: String) -> Self {
        MapKey(s)
    }
}

impl From<&str> for MapKey {
    fn from(s: &str) -> Self {
        MapKey(s.to_string())
    }
}

// Implement CBOR serialization for MapKey.
impl encode::Encode for MapKey {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.str(&self.0)?.ok()
    }
}

// Implement CBOR deserialization for MapKey.
impl<'b> decode::Decode<'b> for MapKey {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let s = d.str()?.to_owned();
        Ok(MapKey(s))
    }
}

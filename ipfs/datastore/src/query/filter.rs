// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;
use std::fmt;

use crate::key::Key;
use crate::query::Entry;

/// Filter is an object that tests io::ResultEntries.
pub trait Filter: dyn_clone::DynClone + fmt::Debug + fmt::Display {
    /// Return whether the entry passes the filter.
    fn filter(&self, entry: &Entry) -> bool;
}

dyn_clone::clone_trait_object!(Filter);

/// Filter comparison operator.
#[doc(hidden)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum FilterOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

impl fmt::Display for FilterOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        match self {
            FilterOp::Equal => f.write_str("=="),
            FilterOp::NotEqual => f.write_str("!="),
            FilterOp::GreaterThan => f.write_str(">"),
            FilterOp::GreaterThanEqual => f.write_str(">="),
            FilterOp::LessThan => f.write_str("<"),
            FilterOp::LessThanEqual => f.write_str("<="),
        }
    }
}

/// A object used to filter data by the key.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FilterKeyCompare {
    key: Key,
    op: FilterOp,
}

impl FilterKeyCompare {
    /// Create a new FilterKeyCompare instance.
    pub fn new(key: Key, op: FilterOp) -> Self {
        Self { key, op }
    }
}

impl fmt::Display for FilterKeyCompare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "KEY {} {}", self.op, self.key)
    }
}

impl Filter for FilterKeyCompare {
    fn filter(&self, entry: &Entry) -> bool {
        match self.op {
            FilterOp::Equal => entry.key == self.key,
            FilterOp::NotEqual => entry.key != self.key,
            FilterOp::GreaterThan => entry.key > self.key,
            FilterOp::GreaterThanEqual => entry.key >= self.key,
            FilterOp::LessThan => entry.key < self.key,
            FilterOp::LessThanEqual => entry.key <= self.key,
        }
    }
}

/// A object used to filter data by the value.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FilterValueCompare {
    value: Vec<u8>,
    op: FilterOp,
}

impl FilterValueCompare {
    /// Create a new FilterValueCompare instance.
    pub fn new<V: Into<Vec<u8>>>(value: V, op: FilterOp) -> Self {
        Self {
            value: value.into(),
            op,
        }
    }
}

impl fmt::Display for FilterValueCompare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "VALUE {} {:?}", self.op, self.value)
    }
}

impl Filter for FilterValueCompare {
    fn filter(&self, entry: &Entry) -> bool {
        let order = entry.value.cmp(&self.value);
        match self.op {
            FilterOp::Equal => order == Ordering::Equal,
            FilterOp::NotEqual => order != Ordering::Equal,
            FilterOp::GreaterThan => order == Ordering::Greater,
            FilterOp::GreaterThanEqual => order == Ordering::Greater || order == Ordering::Equal,
            FilterOp::LessThan => order == Ordering::Less,
            FilterOp::LessThanEqual => order == Ordering::Less || order == Ordering::Equal,
        }
    }
}

/// A object used to filter data by the prefix of key.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FilterKeyPrefix {
    prefix: String,
}

impl FilterKeyPrefix {
    /// Create a new FilterKeyPrefix instance.
    pub fn new<S: Into<String>>(prefix: S) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl fmt::Display for FilterKeyPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "PREFIX({})", self.prefix)
    }
}

impl Filter for FilterKeyPrefix {
    fn filter(&self, entry: &Entry) -> bool {
        entry.key.as_str().starts_with(&self.prefix)
    }
}

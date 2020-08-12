// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;
use std::fmt;

use crate::query::Entry;

/// Order is an object used to order objects
pub trait Order: dyn_clone::DynClone + fmt::Debug + fmt::Display {
    /// Returns an `Ordering` between `lhs` and `rhs`.
    fn compare(&self, lhs: &Entry, rhs: &Entry) -> Ordering;
}

dyn_clone::clone_trait_object!(Order);

/// A object used to order data by the key ascending.
#[derive(Copy, Clone)]
pub struct OrderByKey;

impl fmt::Debug for OrderByKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "OrderByKey: {:p}", &entry_key_ascending)
    }
}

impl fmt::Display for OrderByKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        f.write_str("KEY")
    }
}

impl Order for OrderByKey {
    fn compare(&self, lhs: &Entry, rhs: &Entry) -> Ordering {
        OrderByFunction::new(entry_key_ascending).compare(lhs, rhs)
    }
}

fn entry_key_ascending(lhs: &Entry, rhs: &Entry) -> Ordering {
    lhs.key.cmp(&rhs.key)
}

/// A object used to order data by the key descending.
#[derive(Copy, Clone)]
pub struct OrderByKeyDescending;

impl fmt::Debug for OrderByKeyDescending {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "OrderByKeyDescending: {:p}", &entry_key_descending)
    }
}

impl fmt::Display for OrderByKeyDescending {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        f.write_str("desc(KEY)")
    }
}

impl Order for OrderByKeyDescending {
    fn compare(&self, lhs: &Entry, rhs: &Entry) -> Ordering {
        OrderByFunction::new(entry_key_descending).compare(lhs, rhs)
    }
}

fn entry_key_descending(lhs: &Entry, rhs: &Entry) -> Ordering {
    rhs.key.cmp(&lhs.key)
}

/// A object used to order data by the value ascending.
#[derive(Copy, Clone)]
pub struct OrderByValue;

impl fmt::Debug for OrderByValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "OrderByValue: {:p}", &entry_value_ascending)
    }
}

impl fmt::Display for OrderByValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        f.write_str("VALUE")
    }
}

impl Order for OrderByValue {
    fn compare(&self, lhs: &Entry, rhs: &Entry) -> Ordering {
        OrderByFunction::new(entry_value_ascending).compare(lhs, rhs)
    }
}

fn entry_value_ascending(lhs: &Entry, rhs: &Entry) -> Ordering {
    lhs.value.cmp(&rhs.value)
}

/// A object used to order data by the value descending.
#[derive(Copy, Clone)]
pub struct OrderByValueDescending;

impl fmt::Debug for OrderByValueDescending {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "OrderByValueDescending: {:p}", &entry_value_descending)
    }
}

impl fmt::Display for OrderByValueDescending {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        f.write_str("desc(VALUE)")
    }
}

impl Order for OrderByValueDescending {
    fn compare(&self, lhs: &Entry, rhs: &Entry) -> Ordering {
        OrderByFunction::new(entry_value_descending).compare(lhs, rhs)
    }
}

fn entry_value_descending(lhs: &Entry, rhs: &Entry) -> Ordering {
    rhs.value.cmp(&lhs.value)
}

/// A object used to order data by the function.
#[derive(Clone)]
pub struct OrderByFunction(fn(&Entry, &Entry) -> Ordering);

impl fmt::Debug for OrderByFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        write!(f, "OrderByFunction: {:p}", &self.0)
    }
}

impl OrderByFunction {
    /// Create a new OrderByFunction instance.
    pub fn new(func: fn(&Entry, &Entry) -> Ordering) -> Self {
        Self(func)
    }
}

impl fmt::Display for OrderByFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::io::Result {
        f.write_str("FN")
    }
}

impl Order for OrderByFunction {
    fn compare(&self, lhs: &Entry, rhs: &Entry) -> Ordering {
        (self.0)(lhs, rhs)
    }
}

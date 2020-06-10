// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::cmp::Ordering;

use crate::query::Entry;

///
pub trait Order {
    ///
    fn compare(a: &Entry, b: &Entry) -> Ordering;
}

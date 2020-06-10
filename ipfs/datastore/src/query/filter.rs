// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::query::Entry;

///
pub trait Filter {
    ///
    fn filter(entry: &Entry) -> bool;
}

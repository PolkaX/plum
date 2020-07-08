// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! # AMT
//!
//! See the [specs](https://github.com/ipld/specs/blob/master/data-structures/vector.md) for details.
//!

#![deny(missing_docs)]

mod amt;
mod error;
mod node;
mod root;

pub use self::amt::Amt;

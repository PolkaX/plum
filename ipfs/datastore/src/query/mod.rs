// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod filter;
mod order;

pub use self::filter::{Filter, FilterKeyCompare, FilterKeyPrefix, FilterOp, FilterValueCompare};
pub use self::order::{
    Order, OrderByFunction, OrderByKey, OrderByKeyDescending, OrderByValue, OrderByValueDescending,
};

use std::error;
use std::fmt;
use std::future::Future;
use std::time::Instant;

use crate::key::Key;

/// Query represents storage for any key-value pair.
///
/// tl;dr:
///
/// queries are supported across datastores.
/// Cheap on top of relational dbs, and expensive otherwise.
/// Pick the right tool for the job!
///
/// In addition to the key-value store get and set semantics, datastore
/// provides an interface to retrieve multiple records at a time through
/// the use of queries. The datastore Query model gleans a common set of
/// operations performed when querying. To avoid pasting here years of
/// database research, let’s summarize the operations datastore supports.
///
/// Query Operations, applied in-order:
///
/// * prefix - scope the query to a given path prefix
/// * filters - select a subset of values by applying constraints
/// * orders - sort the results by applying sort conditions, hierarchically.
/// * offset - skip a number of results (for efficient pagination)
/// * limit - impose a numeric limit on the number of results
///
/// DataStore combines these operations into a simple Query class that allows
/// applications to define their constraints in a simple, generic, way without
/// introducing datastore specific calls, languages, etc.
///
/// However, take heed: not all datastores support efficiently performing these
/// operations. Pick a datastore based on your needs. If you need efficient look-ups,
/// go for a simple key/value store. If you need efficient queries, consider an SQL
/// backed datastore.
///
/// Notes:
///
/// * Prefix: When a query filters by prefix, it selects keys that are strict
///   children of the prefix. For example, a prefix "/foo" would select "/foo/bar"
///   but not "/foobar" or "/foo",
/// * Orders: Orders are applied hierarchically. Results are sorted by the first
///   ordering, then entries equal under the first ordering are sorted with the
///   second ordering, etc.
/// * Limits & Offset: Limits and offsets are applied after everything else.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Query {
    pub prefix: String,
    pub filters: Vec<Box<dyn Filter>>,
    pub orders: Vec<Box<dyn Order>>,
    pub limit: usize,
    pub offset: usize,
    pub keys_only: bool,
    pub return_expirations: bool,
    pub return_sizes: bool,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(128);

        result.push_str("SELECT keys");
        if !self.keys_only {
            result.push_str(",vals");
        }
        if self.return_expirations {
            result.push_str(",exps");
        }
        result.push_str(" ");

        if !self.prefix.is_empty() {
            result.push_str(&format!("FROM {} ", self.prefix));
        }
        if !self.filters.is_empty() {
            result.push_str(&format!("FILTER [{}", self.filters[0]));
            for filter in &self.filters[1..] {
                result.push_str(&format!(", {}", filter));
            }
            result.push_str("] ");
        }
        if !self.orders.is_empty() {
            result.push_str(&format!("ORDER [{}", self.orders[0]));
            for order in &self.orders[1..] {
                result.push_str(&format!(", {}", order));
            }
            result.push_str("] ");
        }
        if self.offset > 0 {
            result.push_str(&format!("OFFSET {} ", self.offset));
        }
        if self.limit > 0 {
            result.push_str(&format!("LIMIT {} ", self.limit));
        }

        f.write_str(&result)
    }
}

/// The query result entry.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Entry {
    pub key: Key,
    pub value: Vec<u8>,
    pub expiration: Instant,
    pub size: usize,
}

/// The query result.
pub type QueryResult = Result<Entry, Box<dyn error::Error>>;

///
pub trait QueryResults {
    ///
    fn query(&self) -> Query;

    ///
    fn next(&self) -> Box<dyn Future<Output = QueryResult>>;

    ///
    fn next_sync(&self) -> (QueryResult, bool);

    ///
    fn reset(&self) -> Result<Vec<Entry>, Box<dyn error::Error>>;

    ///
    fn close(&self) -> Result<(), Box<dyn error::Error>>;
}

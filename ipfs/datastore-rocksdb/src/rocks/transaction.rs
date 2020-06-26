// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use smallvec::SmallVec;

/// Database keys.
pub type DBKey = SmallVec<[u8; 32]>;
/// Database value.
pub type DBValue = Vec<u8>;

/// Database operation.
#[doc(hidden)]
#[derive(Clone, PartialEq)]
pub enum DBOp {
    Insert {
        col: String,
        key: DBKey,
        value: DBValue,
    },
    Delete {
        col: String,
        key: DBKey,
    },
    // DeletePrefix { col: String, prefix: DBKey },
}

impl DBOp {
    /// Returns the key associated with this operation.
    pub fn key(&self) -> &[u8] {
        match self {
            DBOp::Insert { key, .. } => key,
            DBOp::Delete { key, .. } => key,
        }
    }

    /// Returns the column associated with this operation.
    pub fn col(&self) -> &str {
        match self {
            DBOp::Insert { col, .. } => col,
            DBOp::Delete { col, .. } => col,
        }
    }
}

/// Write transaction. Batches a sequence of put/delete operations for efficiency.
#[derive(Default, Clone, PartialEq)]
pub struct DBTransaction {
    /// Database operations.
    pub ops: Vec<DBOp>,
}

impl DBTransaction {
    /// Create new transaction.
    pub fn new() -> DBTransaction {
        DBTransaction::with_capacity(256)
    }

    /// Create new transaction with capacity.
    pub fn with_capacity(cap: usize) -> DBTransaction {
        DBTransaction {
            ops: Vec::with_capacity(cap),
        }
    }

    /// Insert a key-value pair in the transaction. Any existing value will be overwritten upon write.
    pub fn put(&mut self, col: &str, key: &[u8], value: Vec<u8>) {
        self.ops.push(DBOp::Insert {
            col: col.to_owned(),
            key: DBKey::from_slice(key),
            value,
        });
    }

    /// Delete value by key.
    pub fn delete(&mut self, col: &str, key: &[u8]) {
        self.ops.push(DBOp::Delete {
            col: col.to_owned(),
            key: DBKey::from_slice(key),
        });
    }

    /// Clear all database operations.
    pub fn clear(&mut self) {
        self.ops.clear();
    }
}

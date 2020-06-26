// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::{HashMap, HashSet};

use rocksdb::{BlockBasedOptions, Options};

use crate::rocks::compact::CompactionProfile;
use crate::rocks::{MiB, DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB, MB};

/// The default name pf a column.
pub const DEFAULT_COLUMN_NAME: &str = "default";

/// Database configuration
#[derive(Clone)]
pub struct DatabaseConfig {
    /// Max number of open files.
    pub max_open_files: i32,
    /// Memory budget (in MiB) used for setting block cache size and
    /// write buffer size for each column including the default one.
    /// If the memory budget of a column is not specified,
    /// `DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB` is used for that column.
    pub memory_budget: HashMap<String, MiB>,
    /// Compaction profile.
    pub compaction: CompactionProfile,
    /// Initialized columns.
    ///
    /// # Safety
    ///
    /// The number of columns must not be zero.
    pub columns: HashSet<String>,
    /// Specify the maximum number of info/debug log files to be kept.
    pub keep_log_file_num: i32,
    /// Enable native RocksDB statistics.
    /// Disabled by default.
    ///
    /// It can have a negative performance impact up to 10% according to
    /// https://github.com/facebook/rocksdb/wiki/Statistics.
    pub enable_statistics: bool,
    /// Open the database as a secondary instance.
    /// Specify a path for the secondary instance of the database.
    /// Secondary instances are read-only and kept updated by tailing the rocksdb MANIFEST.
    /// It is up to the user to call `catch_up_with_primary()` manually to update the secondary db.
    /// Disabled by default.
    ///
    /// `max_open_files` is overridden to always equal `-1`.
    /// May have a negative performance impact on the secondary instance
    /// if the secondary instance reads and applies state changes before the primary instance compacts them.
    /// More info: https://github.com/facebook/rocksdb/wiki/Secondary-instance
    pub secondary: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let mut default_columns = HashSet::new();
        default_columns.insert(DEFAULT_COLUMN_NAME.to_string());

        DatabaseConfig {
            max_open_files: 512,
            memory_budget: HashMap::new(),
            compaction: CompactionProfile::default(),
            columns: default_columns,
            keep_log_file_num: 1,
            enable_statistics: false,
            secondary: None,
        }
    }
}

impl DatabaseConfig {
    /// Create new `DatabaseConfig` with default parameters and specified set of columns.
    /// Note that cache sizes must be explicitly set.
    ///
    /// # Safety
    ///
    /// The number of `columns` must not be zero.
    pub fn with_columns(columns: Vec<String>) -> Self {
        assert!(
            !columns.is_empty(),
            "the number of columns must not be zero"
        );

        Self {
            columns: columns.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Returns the total memory budget in bytes.
    pub fn memory_budget(&self) -> MiB {
        self.columns
            .iter()
            .map(|i| {
                self.memory_budget
                    .get(i.as_str())
                    .unwrap_or(&DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB)
                    * MB
            })
            .sum()
    }

    /// Returns the memory budget of the specified column in bytes.
    pub(crate) fn memory_budget_for_col(&self, col: &str) -> MiB {
        self.memory_budget
            .get(col)
            .unwrap_or(&DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB)
            * MB
    }

    // Get column family configuration with the given block based options.
    pub(crate) fn column_config(&self, block_opts: &BlockBasedOptions, col: &str) -> Options {
        let column_mem_budget = self.memory_budget_for_col(col);
        let mut opts = Options::default();

        opts.set_level_compaction_dynamic_level_bytes(true);
        opts.set_block_based_table_factory(block_opts);
        opts.optimize_level_style_compaction(column_mem_budget);
        opts.set_target_file_size_base(self.compaction.initial_file_size);
        opts.set_compression_per_level(&[]);

        opts
    }
}

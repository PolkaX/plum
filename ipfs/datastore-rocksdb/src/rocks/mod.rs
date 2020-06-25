// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod compact;
mod config;
mod stats;
#[cfg(test)]
mod tests;
mod transaction;

use std::borrow::Borrow;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fs;
use std::io;
use std::path::Path;

use log::warn;
use parking_lot::RwLock;
use rocksdb::{
    BlockBasedOptions, ColumnFamily, ColumnFamilyDescriptor, Error, Options, ReadOptions,
    WriteBatch, WriteOptions, DB,
};

pub use self::compact::CompactionProfile;
pub use self::config::DatabaseConfig;
use self::stats::{parse_rocksdb_stats, RunningDBStats};
pub use self::stats::{IoStats, IoStatsKind, RocksDBStatsValue};
pub use self::transaction::{DBKey, DBOp, DBTransaction, DBValue};

// Used for memory budget.
type MiB = usize;
const KB: usize = 1_024;
const MB: usize = 1_024 * KB;

/// The default column memory budget in MiB.
pub const DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB: MiB = 128;
/// The default memory budget in MiB.
pub const DB_DEFAULT_MEMORY_BUDGET_MB: MiB = 512;

struct DBAndColumns {
    db: DB,
    column_names: HashSet<String>,
}

impl DBAndColumns {
    fn cf(&self, name: &str) -> &ColumnFamily {
        self.db
            .cf_handle(name)
            .expect("the specified column name is correct; qed")
    }
}

#[inline]
fn check_for_corruption<T, P: AsRef<Path>>(path: P, res: Result<T, Error>) -> io::Result<T> {
    if let Err(ref err) = res {
        if is_corrupted(err) {
            warn!(
                "DB corrupted: {}. Repair will be triggered on next restart",
                err
            );
            let _ = fs::File::create(path.as_ref().join(Database::CORRUPTION_FILE_NAME));
        }
    }

    res.map_err(other_io_err)
}

fn is_corrupted(err: &Error) -> bool {
    let err = err.as_ref();
    err.starts_with("Corruption:")
        || err.starts_with("Invalid argument: You have to open all column families")
}

fn other_io_err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

/// Generate the options for RocksDB, based on the given `DatabaseConfig`.
fn generate_options(config: &DatabaseConfig) -> Options {
    let mut opts = Options::default();

    opts.set_report_bg_io_stats(true);
    if config.enable_statistics {
        opts.enable_statistics();
    }
    opts.set_use_fsync(false);
    opts.create_if_missing(true);
    if config.secondary.is_some() {
        opts.set_max_open_files(-1)
    } else {
        opts.set_max_open_files(config.max_open_files);
    }
    opts.set_bytes_per_sync(1 * MB as u64);
    opts.set_keep_log_file_num(1);
    opts.increase_parallelism(cmp::max(1, num_cpus::get() as i32 / 2));

    opts
}

/// Generate the block based options for RocksDB, based on the given `DatabaseConfig`.
fn generate_block_based_options(config: &DatabaseConfig) -> BlockBasedOptions {
    let mut block_opts = BlockBasedOptions::default();
    block_opts.set_block_size(config.compaction.block_size);
    // Set cache size as recommended by
    // https://github.com/facebook/rocksdb/wiki/Setup-Options-and-Basic-Tuning#block-cache-size
    let cache_size = config.memory_budget() / 3;
    if cache_size == 0 {
        block_opts.disable_cache()
    } else {
        block_opts.set_lru_cache(cache_size);
        // "index and filter blocks will be stored in block cache, together with all other data blocks."
        // See: https://github.com/facebook/rocksdb/wiki/Memory-usage-in-RocksDB#indexes-and-filter-blocks
        block_opts.set_cache_index_and_filter_blocks(true);
        // Don't evict L0 filter/index blocks from the cache
        block_opts.set_pin_l0_filter_and_index_blocks_in_cache(true);
    }
    block_opts.set_bloom_filter(10, true);

    block_opts
}

/// Generate the write options for RocksDB.
fn generate_write_options() -> WriteOptions {
    WriteOptions::default()
}

/// Generate the read options for RocksDB.
fn generate_read_options() -> ReadOptions {
    let mut read_opts = ReadOptions::default();
    read_opts.set_verify_checksums(false);
    read_opts
}

/// Key-Value database.
pub struct Database {
    db: RwLock<Option<DBAndColumns>>,
    config: DatabaseConfig,
    path: String,
    opts: Options,
    block_opts: BlockBasedOptions,
    write_opts: WriteOptions,
    read_opts: ReadOptions,
    stats: RunningDBStats,
}

impl Database {
    const CORRUPTION_FILE_NAME: &'static str = "CORRUPTED";

    /// Open database file. Creates if it does not exist.
    ///
    /// # Safety
    ///
    /// The number of `config.columns` must not be zero.
    pub fn open(config: &DatabaseConfig, path: &str) -> io::Result<Database> {
        assert!(
            !config.columns.is_empty(),
            "the number of columns must not be zero"
        );

        let opts = generate_options(config);
        let block_opts = generate_block_based_options(config);
        let write_opts = generate_write_options();
        let read_opts = generate_read_options();

        // attempt database repair if it has been previously marked as corrupted
        let db_corrupted = Path::new(path).join(Database::CORRUPTION_FILE_NAME);
        if db_corrupted.exists() {
            warn!("DB has been previously marked as corrupted, attempting repair");
            DB::repair(&opts, path).map_err(other_io_err)?;
            fs::remove_file(db_corrupted)?;
        }

        let db = if let Some(secondary_path) = &config.secondary {
            Self::open_secondary(&opts, path, secondary_path, config)?
        } else {
            Self::open_primary(&opts, path, config, &block_opts)?
        };
        let column_names = config.columns.clone();

        Ok(Database {
            db: RwLock::new(Some(DBAndColumns { db, column_names })),
            config: config.clone(),
            path: path.to_owned(),
            opts,
            block_opts,
            write_opts,
            read_opts,
            stats: RunningDBStats::new(),
        })
    }

    /// Internal api to open a database in primary mode.
    fn open_primary(
        opts: &Options,
        path: &str,
        config: &DatabaseConfig,
        block_opts: &BlockBasedOptions,
    ) -> io::Result<DB> {
        let cf_descriptions = config
            .columns
            .iter()
            .map(|name| ColumnFamilyDescriptor::new(name, config.column_config(block_opts, name)))
            .collect::<Vec<_>>();

        match DB::open_cf_descriptors(opts, path, cf_descriptions) {
            Ok(db) => Ok(db),
            Err(_) => {
                // retry and create CFs
                match DB::open_cf(opts, path, &[] as &[&str]) {
                    Ok(mut db) => {
                        for name in &config.columns {
                            db.create_cf(name, &config.column_config(block_opts, name))
                                .map_err(other_io_err)?;
                        }
                        Ok(db)
                    }
                    Err(ref err) if is_corrupted(err) => {
                        warn!("DB corrupted: {}, attempting repair", err);
                        DB::repair(opts, path).map_err(other_io_err)?;

                        let cf_descriptions = config
                            .columns
                            .iter()
                            .map(|name| {
                                ColumnFamilyDescriptor::new(
                                    name,
                                    config.column_config(block_opts, name),
                                )
                            })
                            .collect::<Vec<_>>();
                        DB::open_cf_descriptors(opts, path, cf_descriptions).map_err(other_io_err)
                    }
                    Err(err) => Err(other_io_err(err)),
                }
            }
        }
    }

    /// Internal api to open a database in secondary mode.
    /// Secondary database needs a seperate path to store its own logs.
    fn open_secondary(
        opts: &Options,
        path: &str,
        secondary_path: &str,
        config: &DatabaseConfig,
    ) -> io::Result<DB> {
        match DB::open_cf_as_secondary(opts, path, secondary_path, &config.columns) {
            Ok(db) => Ok(db),
            Err(ref err) if is_corrupted(err) => {
                warn!("DB corrupted: {}, attempting repair", err);
                DB::repair(opts, path).map_err(other_io_err)?;
                DB::open_cf_as_secondary(opts, path, secondary_path, &config.columns)
                    .map_err(other_io_err)
            }
            Err(err) => Err(other_io_err(err)),
        }
    }

    /// Helper to create new transaction for this database.
    pub fn transaction(&self) -> DBTransaction {
        DBTransaction::new()
    }

    /// Commit transaction to database.
    pub fn write(&self, txn: DBTransaction) -> io::Result<()> {
        match *self.db.read() {
            Some(ref cfs) => {
                let mut batch = WriteBatch::default();
                let ops = txn.ops;

                self.stats.tally_writes(ops.len() as u64);
                self.stats.tally_transactions(1);

                let mut stats_total_bytes = 0;
                for op in ops {
                    let cf = cfs.cf(op.col());
                    match op {
                        DBOp::Insert { col: _, key, value } => {
                            stats_total_bytes += key.len() + value.len();
                            batch.put_cf(cf, key, value);
                        }
                        DBOp::Delete { col: _, key } => {
                            stats_total_bytes += key.len();
                            batch.delete_cf(cf, key);
                        }
                    }
                }
                self.stats.tally_bytes_written(stats_total_bytes as u64);

                let res = cfs.db.write_opt(batch, &self.write_opts);
                check_for_corruption(&self.path, res)
            }
            None => Err(other_io_err("Database is closed")),
        }
    }

    /// Get value by key.
    pub fn get(&self, col: &str, key: &[u8]) -> io::Result<Option<DBValue>> {
        match *self.db.read() {
            Some(ref cfs) => {
                if !cfs.column_names.contains(col) {
                    return Err(other_io_err("non-existing column"));
                }
                self.stats.tally_reads(1);
                let value = cfs
                    .db
                    .get_pinned_cf_opt(cfs.cf(col), key, &self.read_opts)
                    .map(|r| r.map(|v| v.to_vec()))
                    .map_err(other_io_err);
                match value {
                    Ok(Some(ref val)) => {
                        self.stats.tally_bytes_read((key.len() + val.len()) as u64)
                    }
                    Ok(None) => self.stats.tally_bytes_read(key.len() as u64),
                    _ => {}
                }
                value
            }
            None => Ok(None),
        }
    }

    /// Close the database
    pub fn close(&self) {
        *self.db.write() = None;
    }

    /// The number of column families in the db.
    pub fn num_columns(&self) -> u32 {
        self.db
            .read()
            .as_ref()
            .and_then(|db| {
                if db.column_names.is_empty() {
                    None
                } else {
                    Some(db.column_names.len())
                }
            })
            .map(|n| n as u32)
            .unwrap_or(0)
    }

    /// The number of keys in a column (estimated).
    pub fn num_keys(&self, col: &str) -> io::Result<u64> {
        const ESTIMATE_NUM_KEYS: &str = "rocksdb.estimate-num-keys";
        match *self.db.read() {
            Some(ref cfs) => {
                let cf = cfs.cf(col);
                match cfs.db.property_int_value_cf(cf, ESTIMATE_NUM_KEYS) {
                    Ok(estimate) => Ok(estimate.unwrap_or_default()),
                    Err(err) => Err(other_io_err(err)),
                }
            }
            None => Ok(0),
        }
    }

    /// Add a new column family to the DB.
    pub fn add_column(&self, col: String) -> io::Result<()> {
        match *self.db.write() {
            Some(DBAndColumns {
                ref mut db,
                ref mut column_names,
            }) => {
                let col_opts = self.config.column_config(&self.block_opts, &col);
                db.create_cf(&col, &col_opts).map_err(other_io_err)?;
                column_names.insert(col);
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Remove the column family in the database. The deletion is definitive.
    pub fn remove_column(&self, col: &str) -> io::Result<()> {
        match *self.db.write() {
            Some(DBAndColumns {
                ref mut db,
                ref mut column_names,
            }) => {
                if column_names.remove(col) {
                    db.drop_cf(col.borrow()).map_err(other_io_err)?
                }
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Get RocksDB statistics.
    pub fn get_statistics(&self) -> HashMap<String, RocksDBStatsValue> {
        if let Some(stats) = self.opts.get_statistics() {
            parse_rocksdb_stats(&stats)
        } else {
            HashMap::new()
        }
    }

    /// Query statistics.
    pub fn io_stats(&self, kind: IoStatsKind) -> IoStats {
        let rocksdb_stats = self.get_statistics();
        let cache_hit_count = rocksdb_stats
            .get("block.cache.hit")
            .map(|s| s.count)
            .unwrap_or(0);
        let overall_stats = self.stats.overall();
        let old_cache_hit_count = overall_stats.raw.cache_hit_count;

        self.stats
            .tally_cache_hit_count(cache_hit_count - old_cache_hit_count);

        let taken_stats = match kind {
            IoStatsKind::Overall => self.stats.overall(),
            IoStatsKind::SincePrevious => self.stats.since_previous(),
        };

        let mut stats = IoStats::empty();
        stats.reads = taken_stats.raw.reads;
        stats.bytes_read = taken_stats.raw.bytes_read;
        stats.writes = taken_stats.raw.writes;
        stats.bytes_written = taken_stats.raw.bytes_written;
        stats.transactions = taken_stats.raw.transactions;
        stats.cache_reads = taken_stats.raw.cache_hit_count;
        stats.started = taken_stats.started;
        stats.span = taken_stats.started.elapsed();
        stats
    }
}

// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use parking_lot::RwLock;

/// Statistic kind to query.
pub enum IoStatsKind {
    /// Overall statistics since start.
    Overall,
    /// Statistics since previous query.
    SincePrevious,
}

/// Statistic for the `span` period
#[derive(Debug, Clone)]
pub struct IoStats {
    /// Number of transaction.
    pub transactions: u64,
    /// Number of read operations.
    pub reads: u64,
    /// Number of reads resulted in a read from cache.
    pub cache_reads: u64,
    /// Number of write operations.
    pub writes: u64,
    /// Number of bytes read
    pub bytes_read: u64,
    /// Number of bytes read from cache
    pub cache_read_bytes: u64,
    /// Number of bytes write
    pub bytes_written: u64,
    /// Start of the statistic period.
    pub started: Instant,
    /// Total duration of the statistic period.
    pub span: Duration,
}

impl IoStats {
    /// Empty statistic report.
    pub fn empty() -> Self {
        Self {
            transactions: 0,
            reads: 0,
            cache_reads: 0,
            writes: 0,
            bytes_read: 0,
            cache_read_bytes: 0,
            bytes_written: 0,
            started: Instant::now(),
            span: Duration::default(),
        }
    }

    /// Average batch (transaction) size (writes per transaction)
    pub fn avg_batch_size(&self) -> f64 {
        if self.writes == 0 {
            return 0.0;
        }
        self.transactions as f64 / self.writes as f64
    }

    /// Read operations per second.
    pub fn reads_per_sec(&self) -> f64 {
        if self.span.as_secs_f64() == 0.0 {
            return 0.0;
        }

        self.reads as f64 / self.span.as_secs_f64()
    }

    /// Read bytes operations per second.
    pub fn byte_reads_per_sec(&self) -> f64 {
        if self.span.as_secs_f64() == 0.0 {
            return 0.0;
        }

        self.bytes_read as f64 / self.span.as_secs_f64()
    }

    /// Write operations per second.
    pub fn writes_per_sec(&self) -> f64 {
        if self.span.as_secs_f64() == 0.0 {
            return 0.0;
        }

        self.writes as f64 / self.span.as_secs_f64()
    }

    /// Write bytes operations per second.
    pub fn byte_writes_per_sec(&self) -> f64 {
        if self.span.as_secs_f64() == 0.0 {
            return 0.0;
        }

        self.bytes_written as f64 / self.span.as_secs_f64()
    }

    /// Total number of operations per second.
    pub fn ops_per_sec(&self) -> f64 {
        if self.span.as_secs_f64() == 0.0 {
            return 0.0;
        }

        (self.writes as f64 + self.reads as f64) / self.span.as_secs_f64()
    }

    /// Transactions per second.
    pub fn transactions_per_sec(&self) -> f64 {
        if self.span.as_secs_f64() == 0.0 {
            return 0.0;
        }

        (self.transactions as f64) / self.span.as_secs_f64()
    }

    /// Average transaction size.
    pub fn avg_transaction_size(&self) -> f64 {
        if self.transactions == 0 {
            return 0.0;
        }

        self.bytes_written as f64 / self.transactions as f64
    }

    /// Cache hit ratio.
    pub fn cache_hit_ratio(&self) -> f64 {
        if self.reads == 0 {
            return 0.0;
        }

        self.cache_reads as f64 / self.reads as f64
    }
}

// ============================================================================

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RocksDBStatsValue {
    pub count: u64,
    pub times: Option<RocksDBStatsTimeValue>,
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Default)]
pub struct RocksDBStatsTimeValue {
    /// 50% percentile
    pub p50: f64,
    /// 95% percentile
    pub p95: f64,
    /// 99% percentile
    pub p99: f64,
    /// 100% percentile
    pub p100: f64,
    pub sum: u64,
}

/// Parse statistic of RocksDB line by line.
pub fn parse_rocksdb_stats(stats: &str) -> HashMap<String, RocksDBStatsValue> {
    stats
        .lines()
        .map(|line| parse_rocksdb_stats_row(line.splitn(2, ' ')))
        .collect()
}

fn parse_rocksdb_stats_row<'a, I>(mut iter: I) -> (String, RocksDBStatsValue)
where
    I: Iterator<Item = &'a str>,
{
    const PROOF: &str = "rocksdb statistics format is valid and hasn't changed";
    const SEPARATOR: &str = " : ";
    let key = iter
        .next()
        .expect(PROOF)
        .trim_start_matches("rocksdb.")
        .to_owned();
    let values = iter.next().expect(PROOF);
    let value = if values.starts_with("COUNT") {
        // rocksdb.row.cache.hit COUNT : 0
        RocksDBStatsValue {
            count: values
                .rsplit(SEPARATOR)
                .next()
                .expect(PROOF)
                .parse::<u64>()
                .expect(PROOF),
            times: None,
        }
    } else {
        // rocksdb.db.get.micros P50 : 0.000000 P95 : 0.000000 P99 : 0.000000 P100 : 0.000000 COUNT : 0 SUM : 0
        let values: Vec<&str> = values.split_whitespace().filter(|s| *s != ":").collect();
        // values = ["P50", "0.000000", "P95", "0.000000", "P99", "0.000000", "P100", "0.000000", "COUNT", "0", "SUM", "0"]
        RocksDBStatsValue {
            count: values.get(9).expect(PROOF).parse::<u64>().expect(PROOF),
            times: Some(RocksDBStatsTimeValue {
                p50: values.get(1).expect(PROOF).parse::<f64>().expect(PROOF),
                p95: values.get(3).expect(PROOF).parse::<f64>().expect(PROOF),
                p99: values.get(5).expect(PROOF).parse::<f64>().expect(PROOF),
                p100: values.get(7).expect(PROOF).parse::<f64>().expect(PROOF),
                sum: values.get(11).expect(PROOF).parse::<u64>().expect(PROOF),
            }),
        }
    };
    (key, value)
}

#[doc(hidden)]
#[derive(Copy, Clone, Default)]
pub struct RawDBStats {
    pub reads: u64,
    pub bytes_read: u64,
    pub writes: u64,
    pub bytes_written: u64,
    pub transactions: u64,
    pub cache_hit_count: u64,
}

impl RawDBStats {
    fn combine(&self, other: &RawDBStats) -> Self {
        RawDBStats {
            reads: self.reads + other.reads,
            bytes_read: self.bytes_read + other.bytes_read,
            writes: self.writes + other.writes,
            bytes_written: self.bytes_written + other.bytes_written,
            transactions: self.transactions + other.transactions,
            cache_hit_count: self.cache_hit_count + other.cache_hit_count,
        }
    }
}

struct OverallDBStats {
    stats: RawDBStats,
    last_taken: Instant,
    started: Instant,
}

impl OverallDBStats {
    fn new() -> Self {
        Self {
            stats: RawDBStats::default(),
            last_taken: Instant::now(),
            started: Instant::now(),
        }
    }
}

#[doc(hidden)]
pub struct RunningDBStats {
    reads: AtomicU64,
    bytes_read: AtomicU64,
    writes: AtomicU64,
    bytes_written: AtomicU64,
    transactions: AtomicU64,
    cache_hit_count: AtomicU64,
    overall: RwLock<OverallDBStats>,
}

#[doc(hidden)]
pub struct TakenDBStats {
    pub raw: RawDBStats,
    pub started: Instant,
}

impl RunningDBStats {
    /// Create a new running database statistic.
    pub fn new() -> Self {
        Self {
            reads: AtomicU64::new(0),
            bytes_read: AtomicU64::new(0),
            writes: AtomicU64::new(0),
            bytes_written: AtomicU64::new(0),
            transactions: AtomicU64::new(0),
            cache_hit_count: AtomicU64::new(0),
            overall: RwLock::new(OverallDBStats::new()),
        }
    }

    /// Tally reads.
    pub fn tally_reads(&self, val: u64) {
        self.reads.fetch_add(val, Ordering::Relaxed);
    }

    /// Tally bytes read.
    pub fn tally_bytes_read(&self, val: u64) {
        self.bytes_read.fetch_add(val, Ordering::Relaxed);
    }

    /// Tally writes.
    pub fn tally_writes(&self, val: u64) {
        self.writes.fetch_add(val, Ordering::Relaxed);
    }

    /// Tally bytes written.
    pub fn tally_bytes_written(&self, val: u64) {
        self.bytes_written.fetch_add(val, Ordering::Relaxed);
    }

    /// Tally transactions.
    pub fn tally_transactions(&self, val: u64) {
        self.transactions.fetch_add(val, Ordering::Relaxed);
    }

    /// Tally cache hit count.
    pub fn tally_cache_hit_count(&self, val: u64) {
        self.cache_hit_count.fetch_add(val, Ordering::Relaxed);
    }

    /// Overall statistics since start.
    pub fn overall(&self) -> TakenDBStats {
        let mut overall = self.overall.write();

        let current = self.take_current();
        overall.stats = overall.stats.combine(&current);
        let stats = TakenDBStats {
            raw: current,
            started: overall.last_taken,
        };
        overall.last_taken = Instant::now();

        stats
    }

    fn take_current(&self) -> RawDBStats {
        RawDBStats {
            reads: self.reads.swap(0, Ordering::Relaxed),
            bytes_read: self.bytes_read.swap(0, Ordering::Relaxed),
            writes: self.writes.swap(0, Ordering::Relaxed),
            bytes_written: self.bytes_written.swap(0, Ordering::Relaxed),
            transactions: self.transactions.swap(0, Ordering::Relaxed),
            cache_hit_count: self.cache_hit_count.swap(0, Ordering::Relaxed),
        }
    }

    /// Statistics since previous query.
    pub fn since_previous(&self) -> TakenDBStats {
        let overall = self.overall.read();
        let current = self.peek_current();
        TakenDBStats {
            raw: overall.stats.combine(&current),
            started: overall.started,
        }
    }

    fn peek_current(&self) -> RawDBStats {
        RawDBStats {
            reads: self.reads.load(Ordering::Relaxed),
            bytes_read: self.bytes_read.load(Ordering::Relaxed),
            writes: self.writes.load(Ordering::Relaxed),
            bytes_written: self.bytes_written.load(Ordering::Relaxed),
            transactions: self.transactions.load(Ordering::Relaxed),
            cache_hit_count: self.cache_hit_count.load(Ordering::Relaxed),
        }
    }
}

#[test]
fn test_stats_parser() {
    let raw = r#"rocksdb.row.cache.hit COUNT : 1
rocksdb.db.get.micros P50 : 2.000000 P95 : 3.000000 P99 : 4.000000 P100 : 5.000000 COUNT : 0 SUM : 15
"#;
    let stats = parse_rocksdb_stats(raw);
    assert_eq!(stats["row.cache.hit"].count, 1);
    assert!(stats["row.cache.hit"].times.is_none());
    assert_eq!(stats["db.get.micros"].count, 0);
    let get_times = stats["db.get.micros"].times.unwrap();
    assert_eq!(get_times.sum, 15);
    assert!((get_times.p50 - 2.0).abs() < f64::EPSILON);
    assert!((get_times.p95 - 3.0).abs() < f64::EPSILON);
    assert!((get_times.p99 - 4.0).abs() < f64::EPSILON);
    assert!((get_times.p100 - 5.0).abs() < f64::EPSILON);
}

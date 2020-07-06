// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

use super::{Database, DatabaseConfig, DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB, MB};

fn open_temp_db(columns: Vec<String>) -> io::Result<Database> {
    let tempdir = tempfile::Builder::new().prefix("").tempdir()?;
    let config = DatabaseConfig::with_columns(columns);
    Database::open(
        &config,
        tempdir
            .path()
            .to_str()
            .expect("tempdir path is valid unicode"),
    )
}

#[test]
fn get_fails_with_non_existing_column() -> io::Result<()> {
    let db = open_temp_db(vec!["0".into()])?;
    assert!(db.get("1", &[]).is_err());
    Ok(())
}

#[test]
fn put_and_get() -> io::Result<()> {
    let db = open_temp_db(vec!["0".into()])?;
    let key1 = b"key1";

    let mut transaction = db.transaction();
    transaction.put("0", key1, b"horse".to_vec());
    db.write(&transaction)?;

    assert_eq!(db.get("0", key1)?.unwrap(), b"horse");
    Ok(())
}

#[test]
fn delete_and_get() -> io::Result<()> {
    let db = open_temp_db(vec!["0".into()])?;
    let key1 = b"key1";

    let mut transaction = db.transaction();
    transaction.put("0", key1, b"horse".to_vec());
    db.write(&transaction)?;
    assert_eq!(&*db.get("0", key1)?.unwrap(), b"horse");

    let mut transaction = db.transaction();
    transaction.delete("0", key1);
    db.write(&transaction)?;
    assert!(db.get("0", key1)?.is_none());
    Ok(())
}

#[test]
fn complex() -> io::Result<()> {
    let db = open_temp_db(vec!["0".into()])?;

    let key1 = b"02c69be41d0b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc";
    let key2 = b"03c69be41d0b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc";
    let key3 = b"04c00000000b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc";
    let key4 = b"04c01111110b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc";
    let key5 = b"04c02222220b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc";

    let mut batch = db.transaction();
    batch.put("0", key1, b"cat".to_vec());
    batch.put("0", key2, b"dog".to_vec());
    batch.put("0", key3, b"caterpillar".to_vec());
    batch.put("0", key4, b"beef".to_vec());
    batch.put("0", key5, b"fish".to_vec());
    db.write(&batch)?;

    assert_eq!(&*db.get("0", key1)?.unwrap(), b"cat");

    // let contents: Vec<_> = db.iter(0).into_iter().collect();
    // assert_eq!(contents.len(), 5);
    // assert_eq!(contents[0].0.to_vec(), key1.to_vec());
    // assert_eq!(&*contents[0].1, b"cat");
    // assert_eq!(contents[1].0.to_vec(), key2.to_vec());
    // assert_eq!(&*contents[1].1, b"dog");

    // let mut prefix_iter = db.iter_with_prefix(0, b"04c0");
    // assert_eq!(*prefix_iter.next().unwrap().1, b"caterpillar"[..]);
    // assert_eq!(*prefix_iter.next().unwrap().1, b"beef"[..]);
    // assert_eq!(*prefix_iter.next().unwrap().1, b"fish"[..]);

    let mut batch = db.transaction();
    batch.delete("0", key1);
    db.write(&batch)?;

    assert!(db.get("0", key1)?.is_none());

    let mut batch = db.transaction();
    batch.put("0", key1, b"cat".to_vec());
    db.write(&batch)?;

    let mut transaction = db.transaction();
    transaction.put("0", key3, b"elephant".to_vec());
    transaction.delete("0", key1);
    db.write(&transaction)?;
    assert!(db.get("0", key1)?.is_none());
    assert_eq!(&*db.get("0", key3)?.unwrap(), b"elephant");

    // assert_eq!(&*db.get_by_prefix(0, key3).unwrap(), b"elephant");
    // assert_eq!(&*db.get_by_prefix(0, key2).unwrap(), b"dog");

    let mut transaction = db.transaction();
    transaction.put("0", key1, b"horse".to_vec());
    transaction.delete("0", key3);
    db.write(&transaction)?;
    assert!(db.get("0", key3)?.is_none());
    assert_eq!(&*db.get("0", key1)?.unwrap(), b"horse");

    assert!(db.get("0", key3)?.is_none());
    assert_eq!(&*db.get("0", key1)?.unwrap(), b"horse");
    Ok(())
}

/*
#[test]
fn stats() -> io::Result<()> {
    let db = open_temp_db(vec!["0".into(), "1".into(), "2".into()])?;
    let key1 = b"kkk";
    let mut batch = db.transaction();
    batch.put("0", key1, key1);
    batch.put("1", key1, key1);
    batch.put("2", key1, key1);

    for _ in 0..10 {
        db.get("0", key1)?;
    }

    db.write(batch)?;

    let io_stats = db.io_stats(IoStatsKind::SincePrevious);
    assert_eq!(io_stats.transactions, 1);
    assert_eq!(io_stats.writes, 3);
    assert_eq!(io_stats.bytes_written, 18);
    assert_eq!(io_stats.reads, 10);
    assert_eq!(io_stats.bytes_read, 30);

    let new_io_stats = db.io_stats(IoStatsKind::SincePrevious);
    // Since we taken previous statistic period,
    // this is expected to be totally empty.
    assert_eq!(new_io_stats.transactions, 0);

    // but the overall should be there
    let new_io_stats = db.io_stats(IoStatsKind::Overall);
    assert_eq!(new_io_stats.bytes_written, 18);

    let mut batch = db.transaction();
    batch.delete("0", key1);
    batch.delete("1", key1);
    batch.delete("2", key1);

    // transaction is not committed yet
    assert_eq!(db.io_stats(IoStatsKind::SincePrevious).writes, 0);

    db.write(batch)?;
    // now it is, and delete is counted as write
    assert_eq!(db.io_stats(IoStatsKind::SincePrevious).writes, 3);
    Ok(())
}
*/

#[test]
fn secondary_db_get() -> io::Result<()> {
    let primary = tempfile::Builder::new().prefix("").tempdir()?;
    let config = DatabaseConfig::with_columns(vec!["0".into()]);
    let db = Database::open(
        &config,
        primary
            .path()
            .to_str()
            .expect("tempdir path is valid unicode"),
    )?;

    let key1 = b"key1";
    let mut transaction = db.transaction();
    transaction.put("0", key1, b"horse".to_vec());
    db.write(&transaction)?;

    let mut columns = HashSet::new();
    columns.insert("0".to_string());
    let config = DatabaseConfig {
        columns,
        secondary: tempfile::Builder::new()
            .prefix("")
            .tempdir()?
            .path()
            .to_str()
            .map(|s| s.to_string()),
        ..Default::default()
    };
    let second_db = Database::open(
        &config,
        primary
            .path()
            .to_str()
            .expect("tempdir path is valid unicode"),
    )?;
    assert_eq!(&*second_db.get("0", key1)?.unwrap(), b"horse");
    Ok(())
}

#[test]
#[should_panic]
fn db_config_with_zero_columns() {
    let _cfg = DatabaseConfig::with_columns(vec![]);
}

#[test]
#[should_panic]
fn open_db_with_zero_columns() {
    let cfg = DatabaseConfig {
        columns: HashSet::new(),
        ..Default::default()
    };
    let _db = Database::open(&cfg, "");
}

#[test]
fn add_columns() -> io::Result<()> {
    let tempdir = tempfile::Builder::new().prefix("add_columns").tempdir()?;

    // open 1, add 4.
    {
        let config_1 = DatabaseConfig::default();
        let db = Database::open(&config_1, tempdir.path().to_str().unwrap())?;
        assert_eq!(db.num_columns(), 1);

        for i in 0..5 {
            db.add_column(i.to_string())?;
            assert_eq!(db.num_columns(), 1 + 1 + i);
        }
    }

    // reopen as 5.
    {
        let config_5 = DatabaseConfig::with_columns(vec![
            "0".into(),
            "1".into(),
            "2".into(),
            "3".into(),
            "4".into(),
        ]);
        let db = Database::open(&config_5, tempdir.path().to_str().unwrap())?;
        assert_eq!(db.num_columns(), 5);
    }

    Ok(())
}
/*
#[test]
fn remove_columns() -> io::Result<()> {
    let tempdir = tempfile::Builder::new()
        .prefix("remove_columns")
        .tempdir()?;

    // open 5, remove 4.
    {
        let config_5 = DatabaseConfig::with_columns(vec![
            "0".into(),
            "1".into(),
            "2".into(),
            "3".into(),
            "4".into(),
        ]);
        let db = Database::open(&config_5, tempdir.path().to_str().unwrap())?;
        assert_eq!(db.num_columns(), 5);

        for i in (1..5).rev() {
            db.remove_column(&i.to_string())?;
            assert_eq!(db.num_columns(), i);
        }
    }

    // reopen as 1.
    {
        let config_1 = DatabaseConfig::default();
        let db = Database::open(&config_1, tempdir.path().to_str().unwrap())?;
        assert_eq!(db.num_columns(), 1);
    }

    Ok(())
}
*/
#[test]
fn test_num_keys() -> io::Result<()> {
    let db = open_temp_db(vec!["0".into()])?;

    assert_eq!(
        db.num_keys("0").unwrap(),
        0,
        "database is empty after creation"
    );
    let key1 = b"beef";
    let mut batch = db.transaction();
    batch.put("0", key1, key1.to_vec());
    db.write(&batch)?;
    assert_eq!(
        db.num_keys("0").unwrap(),
        1,
        "adding a key increases the count"
    );
    Ok(())
}

#[test]
fn default_memory_budget() {
    let config = DatabaseConfig::default();
    assert_eq!(config.columns.len(), 1);
    assert_eq!(
        config.memory_budget(),
        DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB * MB,
        "total memory budget is default"
    );
    assert_eq!(
        config.memory_budget_for_col("0"),
        DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB * MB,
        "total memory budget for column 0 is the default"
    );
    assert_eq!(
        config.memory_budget_for_col("999"),
        DB_DEFAULT_COLUMN_MEMORY_BUDGET_MB * MB,
        "total memory budget for any column is the default"
    );
}

#[test]
fn memory_budget() {
    let mut config = DatabaseConfig::with_columns(vec!["0".into(), "1".into(), "2".into()]);
    config.memory_budget = vec![
        ("0".to_string(), 10),
        ("1".to_string(), 15),
        ("2".to_string(), 20),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        config.memory_budget(),
        45 * MB,
        "total budget is the sum of the column budget"
    );
}

#[test]
fn rocksdb_settings() -> io::Result<()> {
    const NUM_COLS: usize = 2;
    let mut cfg = DatabaseConfig {
        enable_statistics: true,
        ..DatabaseConfig::with_columns(vec!["0".into(), "1".into()])
    };
    cfg.max_open_files = 123; // is capped by the OS fd limit (typically 1024)
    cfg.compaction.block_size = 323232;
    cfg.compaction.initial_file_size = 102030;
    cfg.memory_budget = vec![("0".to_string(), 30), ("1".to_string(), 300)]
        .into_iter()
        .collect();

    let db_path = tempfile::Builder::new().prefix("config_test").tempdir()?;
    let db = Database::open(&cfg, db_path.path().to_str().unwrap())?;
    let mut rocksdb_log = File::open(format!("{}/LOG", db_path.path().to_str().unwrap()))?;
    let mut settings = String::new();
    let statistics = db.get_statistics();
    assert!(statistics.contains_key("block.cache.hit"));

    rocksdb_log.read_to_string(&mut settings).unwrap();
    // Check column count
    assert!(
        settings.contains("Options for column family [default]"),
        "no default col"
    );
    assert!(settings.contains("Options for column family [0]"), "no 0");
    assert!(settings.contains("Options for column family [1]"), "no 1");

    // Check max_open_files
    assert!(settings.contains("max_open_files: 123"));

    // Check block size
    assert!(settings.contains(" block_size: 323232"));

    // LRU cache (default column)
    assert!(settings.contains("block_cache_options:\n    capacity : 8388608"));
    // LRU cache for non-default columns is ⅓ of memory budget (including default column)
    let lru_size = (330 * MB) / 3;
    let needle = format!("block_cache_options:\n    capacity : {}", lru_size);
    let lru = settings.match_indices(&needle).count();
    assert_eq!(lru, NUM_COLS);

    // Index/filters share cache
    let include_indexes = settings.matches("cache_index_and_filter_blocks: 1").count();
    assert_eq!(include_indexes, NUM_COLS);
    // Pin index/filters on L0
    let pins = settings
        .matches("pin_l0_filter_and_index_blocks_in_cache: 1")
        .count();
    assert_eq!(pins, NUM_COLS);

    // Check target file size, aka initial file size
    let l0_sizes = settings.matches("target_file_size_base: 102030").count();
    assert_eq!(l0_sizes, NUM_COLS);
    // The default column uses the default of 64Mb regardless of the setting.
    assert!(settings.contains("target_file_size_base: 67108864"));

    // Check compression settings
    let snappy_compression = settings.matches("Options.compression: Snappy").count();
    // All columns use Snappy
    assert_eq!(snappy_compression, NUM_COLS + 1);
    // …even for L7
    let snappy_bottommost = settings
        .matches("Options.bottommost_compression: Disabled")
        .count();
    assert_eq!(snappy_bottommost, NUM_COLS + 1);

    // 7 levels
    let levels = settings.matches("Options.num_levels: 7").count();
    assert_eq!(levels, NUM_COLS + 1);

    // Don't fsync every store
    assert!(settings.contains("Options.use_fsync: 0"));

    // We're using the old format
    assert!(settings.contains("format_version: 2"));

    Ok(())
}

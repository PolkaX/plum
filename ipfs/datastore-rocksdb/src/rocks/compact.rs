// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::path::Path;

use crate::rocks::{KB, MB};

/// Compaction profile for the database settings
/// Note, that changing these parameters may trigger
/// the compaction process of RocksDB on startup.
/// https://github.com/facebook/rocksdb/wiki/Leveled-Compaction#level_compaction_dynamic_level_bytes-is-true
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CompactionProfile {
    /// L0-L1 target file size
    /// The minimum size should be calculated in accordance with the
    /// number of levels and the expected size of the database.
    pub initial_file_size: u64,
    /// block size
    pub block_size: usize,
}

impl Default for CompactionProfile {
    /// Default profile suitable for most storage
    fn default() -> CompactionProfile {
        CompactionProfile::ssd()
    }
}

impl CompactionProfile {
    /// Attempt to determine the best profile automatically, only Linux for now.
    #[cfg(target_os = "linux")]
    pub fn auto(db_path: &Path) -> CompactionProfile {
        use std::{fs::File, io::Read, process::Command};

        let hdd_check_file = db_path
            .to_str()
            .and_then(|path_str| Command::new("df").arg(path_str).output().ok())
            .and_then(|df_res| {
                if df_res.status.success() {
                    Some(df_res.stdout)
                } else {
                    None
                }
            })
            .and_then(rotational_from_df_output);
        // Read out the file and match compaction profile.
        if let Some(hdd_check) = hdd_check_file {
            if let Ok(mut file) = File::open(hdd_check.as_path()) {
                let mut buffer = [0; 1];
                if file.read_exact(&mut buffer).is_ok() {
                    // 0 means not rotational.
                    if buffer == [b'0'] {
                        return Self::ssd();
                    }
                    // 1 means rotational.
                    if buffer == [b'1'] {
                        return Self::hdd();
                    }
                }
            }
        }
        // Fallback if drive type was not determined.
        Self::default()
    }

    /// Just default for other platforms.
    #[cfg(not(target_os = "linux"))]
    pub fn auto(_db_path: &Path) -> CompactionProfile {
        Self::default()
    }

    /// Default profile suitable for SSD storage
    pub fn ssd() -> CompactionProfile {
        CompactionProfile {
            initial_file_size: 64 * MB as u64,
            block_size: 16 * KB,
        }
    }

    /// Slow HDD compaction profile
    pub fn hdd() -> CompactionProfile {
        CompactionProfile {
            initial_file_size: 256 * MB as u64,
            block_size: 64 * KB,
        }
    }
}

/// Given output of df command return Linux rotational flag file path.
#[cfg(target_os = "linux")]
fn rotational_from_df_output<O: AsRef<[u8]>>(df_out: O) -> Option<std::path::PathBuf> {
    std::str::from_utf8(df_out.as_ref())
        .ok()
        // Get the drive name.
        .and_then(|df_str| {
            regex::Regex::new(r"/dev/(sd[:alpha:]{1,2}|nvme\dn\dp\d)")
                .ok()
                .and_then(|re| re.captures(df_str))
                .and_then(|captures| captures.get(1))
        })
        // Generate path e.g. /sys/block/sda/queue/rotational
        .map(|drive_path| {
            let mut p = std::path::PathBuf::from("/sys/block");
            p.push(drive_path.as_str());
            p.push("queue/rotational");
            p
        })
}

#[test]
#[cfg(target_os = "linux")]
fn df_to_rotational() {
    use std::path::PathBuf;
    // Example df output.
    let example_df = "\
    Filesystem     1K-blocks     Used Available Use% Mounted on\n\
    /dev/sda1       61409300 38822236  19444616  67% /";
    let expected_output = Some(PathBuf::from("/sys/block/sda/queue/rotational"));
    assert_eq!(rotational_from_df_output(example_df), expected_output);

    let example_df = "\
    Filesystem     1K-blocks      Used Available Use% Mounted on\n\
    /dev/nvme0n1p2 211887896 154833728  46221160  78% /";
    let expected_output = Some(PathBuf::from("/sys/block/nvme0n1p2/queue/rotational"));
    assert_eq!(rotational_from_df_output(example_df), expected_output);
}

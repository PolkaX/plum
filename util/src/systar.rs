// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use log::{debug, warn};
use crate::error::Error;
use std::fs;

pub type Result<T> = std::result::Result<T, Error>;

pub fn extract_tar(dir: &str) -> Result<()>{
    fs::create_dir_all(dir)?;

    Ok(())
}
//pub fn TarDirectory() {}
//pub fn writeTarDirectory() {}
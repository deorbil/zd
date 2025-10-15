use std::path::PathBuf;

use anyhow::{Error, Result};

pub fn get_zd_dir() -> Result<PathBuf> {
    std::env::var("ZD_DIR")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|dir| PathBuf::from(dir).join(".zd")))
        .map_err(Error::from)
}

pub fn get_plugins_dir() -> Result<PathBuf> {
    get_zd_dir().map(|dir| dir.join("plugins"))
}

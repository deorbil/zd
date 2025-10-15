use std::path::PathBuf;

use anyhow::Result;

pub fn create_plugins_dir() -> Result<PathBuf> {
    let dir = get_plugins_dir()?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_home_dir() -> Result<PathBuf> {
    Ok(std::env::var("HOME").map(PathBuf::from)?)
}

pub fn get_zd_dir() -> Result<PathBuf> {
    Ok(match std::env::var("ZD_DIR").map(PathBuf::from) {
        Ok(dir) => dir,
        Err(_) => get_home_dir()?.join(".zd"),
    })
}

pub fn get_plugins_dir() -> Result<PathBuf> {
    Ok(get_zd_dir()?.join("plugins"))
}

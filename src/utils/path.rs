use std::path::PathBuf;

use anyhow::Result;

pub fn create_plugins_dir() -> Result<PathBuf> {
    let dir = get_plugins_dir()?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_home_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(std::env::var("HOME")?))
}

pub fn get_zd_dir() -> Result<PathBuf> {
    Ok(match std::env::var("ZD_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => get_home_dir()?.join(".zd"),
    })
}

pub fn get_plugins_dir() -> Result<PathBuf> {
    Ok(get_zd_dir()?.join("plugins"))
}

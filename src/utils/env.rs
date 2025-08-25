use anyhow::Result;
use std::path::PathBuf;

pub fn get_zd_dir() -> Result<PathBuf> {
    let dir = match std::env::var("ZD_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from(std::env::var("HOME")?).join(".zd"),
    };
    Ok(dir)
}

pub fn get_plugins_dir() -> Result<PathBuf> {
    let dir = get_zd_dir()?.join("plugins");
    Ok(dir)
}

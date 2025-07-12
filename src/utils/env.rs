use std::env::VarError;
use std::path::PathBuf;

pub fn get_zd_dir() -> Result<PathBuf, VarError> {
    std::env::var("ZD_DIR")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|dir| PathBuf::from(dir).join(".zd")))
}

pub fn get_plugins_dir() -> Result<PathBuf, VarError> {
    get_zd_dir().map(|dir| dir.join("plugins"))
}

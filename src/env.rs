use std::path::PathBuf;

pub fn get_zd_dir() -> PathBuf {
    std::env::var("ZD_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|dir| PathBuf::from(dir).join(".zd"))
                .unwrap()
        })
}

use std::fs::DirEntry;
use std::iter::Peekable;
use std::path::Path;

use anyhow::Result;

pub fn get_dirs(path: &Path) -> Result<Peekable<impl Iterator<Item = DirEntry>>> {
    Ok(std::fs::read_dir(path)?
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .peekable())
}

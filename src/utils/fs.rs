use std::fs::DirEntry;
use std::path::Path;

use anyhow::Result;

pub fn get_dirs<P>(path: P) -> Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    Ok(std::fs::read_dir(path)?
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .collect())
}

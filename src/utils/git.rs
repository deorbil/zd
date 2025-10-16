use std::path::Path;
use std::process::Command;

use anyhow::Result;

pub fn clone(dir: &Path, url: &str) -> Result<()> {
    Command::new("git")
        .current_dir(dir)
        .args(["clone", "--quiet", url])
        .status()?;
    Ok(())
}

pub fn pull(dir: &Path) -> Result<()> {
    Command::new("git")
        .current_dir(dir)
        .args(["pull", "--ff-only", "--quiet"])
        .status()?;
    Ok(())
}

use std::path::Path;
use std::process::Command;

use anyhow::Result;

pub fn clone<P, S>(dir: P, url: S, name: S) -> Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    Command::new("git")
        .current_dir(dir)
        .args(["clone", "--quiet", url.as_ref(), name.as_ref()])
        .status()?;
    Ok(())
}

pub fn pull<P>(dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    Command::new("git")
        .current_dir(dir)
        .args(["pull", "--ff-only", "--quiet"])
        .status()?;
    Ok(())
}

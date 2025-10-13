use std::path::Path;
use std::process::Command;

use anyhow::Result;

pub fn run<F>(f: F) -> Result<()>
where
    F: FnOnce(&mut Command) -> &mut Command,
{
    let mut git = Command::new("git");
    f(&mut git);
    git.status()?;
    Ok(())
}

pub fn clone(url: &str, path: &Path) -> Result<()> {
    run(|git| git.current_dir(path).args(["clone", url]))
}

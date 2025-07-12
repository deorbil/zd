use anyhow::Result;
use clap::Parser;
use git2::Repository;

use crate::utils::env;

#[derive(Parser)]
pub struct Add {
    name: String,
    repository: String,
}

impl Add {
    pub fn run(&self) -> Result<()> {
        let dir = env::get_zd_dir()?.join("plugins").join(&self.name);
        std::fs::create_dir_all(&dir)?;
        Repository::clone(&self.repository, &dir)?;
        Ok(())
    }
}

use anyhow::Result;
use clap::Parser;
use git2::Repository;

use crate::utils;

#[derive(Parser)]
pub struct Add {
    url: String,
}

impl Add {
    pub fn run(&self) -> Result<()> {
        let name = utils::git::get_name_from_url(&self.url);
        let dir = utils::env::get_plugins_dir()?.join(name);
        std::fs::create_dir_all(&dir)?;
        Repository::clone(&self.url, &dir)?;
        Ok(())
    }
}

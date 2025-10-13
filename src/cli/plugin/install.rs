use anyhow::Result;
use clap::Parser;
use git2::Repository;

use crate::utils;

#[derive(Parser)]
pub struct Install {
    /// URL of the plugin to be installed
    url: String,
}

impl Install {
    pub fn run(&self) -> Result<()> {
        let name = utils::url::get_name(&self.url);
        let url = utils::url::normalize(&self.url);
        let dir = utils::path::get_plugin_dir(&name)?;
        std::fs::create_dir_all(&dir)?;
        Repository::clone(&url, &dir)?;
        Ok(())
    }
}

use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Install {
    /// URL of the plugin to be installed
    url: String,
}

impl Install {
    pub fn run(&self) -> Result<()> {
        let url = utils::url::normalize(&self.url);
        let dir = utils::path::get_plugins_dir()?;
        std::fs::create_dir_all(&dir)?;
        utils::git::clone(&url, &dir)?;
        Ok(())
    }
}

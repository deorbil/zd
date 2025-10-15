use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Install {
    /// Plugins to be installed
    #[arg(required = true)]
    plugins: Vec<String>,
}

impl Install {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::get_plugins_dir()?;
        std::fs::create_dir_all(&dir)?;

        for plugin in &self.plugins {
            println!("Installing {}...", plugin);
            let url = utils::url::normalize(plugin);
            utils::git::clone(&url, &dir)?;
        }

        Ok(())
    }
}

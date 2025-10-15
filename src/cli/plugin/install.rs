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
        let dir = utils::path::create_plugins_dir()?;

        for plugin in &self.plugins {
            println!("Installing {}...", plugin);
            let url = utils::url::normalize(plugin);
            utils::git::clone(&url, &dir)?;
        }

        Ok(())
    }
}

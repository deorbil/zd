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
            let name = utils::url::get_name(plugin);
            let url = utils::url::normalize(plugin);
            println!("Installing {}...", name);
            utils::git::clone(&dir, &url, &name)?;
        }

        Ok(())
    }
}

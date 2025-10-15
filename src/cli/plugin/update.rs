use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Update {
    /// Plugins to be updated
    plugins: Option<Vec<String>>,
}

impl Update {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::get_plugins_dir()?;
        if let Some(plugins) = &self.plugins {
            for plugin in plugins {
                println!("Updating {}...", plugin);
                utils::git::pull(&dir.join(plugin))?;
            }
        } else {
            let entries = std::fs::read_dir(&dir)?;
            for entry in entries.flatten().filter(|entry| entry.path().is_dir()) {
                println!("Updating {}...", entry.file_name().to_string_lossy());
                utils::git::pull(&entry.path())?;
            }
        }
        Ok(())
    }
}

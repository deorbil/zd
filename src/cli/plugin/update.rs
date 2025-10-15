use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Update {
    /// Name of the plugin to be updated
    name: Option<String>,
}

impl Update {
    pub fn run(&self) -> Result<()> {
        if let Some(name) = &self.name {
            println!("Updating {}...", name);
            let dir = utils::path::get_plugin_dir(name)?;
            utils::git::pull(&dir)?;
        } else {
            let dir = utils::path::get_plugins_dir()?;
            let entries = std::fs::read_dir(&dir)?;
            for entry in entries.flatten().filter(|entry| entry.path().is_dir()) {
                println!("Updating {}...", entry.file_name().to_string_lossy());
                utils::git::pull(&entry.path())?;
            }
        }
        Ok(())
    }
}

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
        let dir = utils::path::create_plugins_dir()?;

        if let Some(plugins) = &self.plugins {
            for plugin in plugins {
                println!("Updating {}...", plugin);
                utils::git::pull(dir.join(plugin))?;
            }
        } else {
            let entries = utils::fs::get_dirs(&dir)?;
            if !entries.is_empty() {
                for entry in entries {
                    println!("Updating {}...", entry.file_name().to_string_lossy());
                    utils::git::pull(entry.path())?;
                }
            } else {
                println!("No installed plugins.");
            }
        }

        Ok(())
    }
}

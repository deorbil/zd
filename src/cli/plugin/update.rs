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
        std::fs::create_dir_all(&dir)?;

        if let Some(plugins) = &self.plugins {
            for plugin in plugins {
                println!("Updating {}...", plugin);
                utils::git::pull(&dir.join(plugin))?;
            }
        } else {
            let mut entries = std::fs::read_dir(&dir)?
                .flatten()
                .filter(|entry| entry.path().is_dir())
                .peekable();

            if entries.peek().is_some() {
                for entry in entries {
                    println!("Updating {}...", entry.file_name().to_string_lossy());
                    utils::git::pull(&entry.path())?;
                }
            } else {
                println!("No installed plugins.");
            }
        }

        Ok(())
    }
}

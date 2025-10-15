use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Uninstall {
    /// Plugins to be uninstalled
    #[arg(required = true)]
    plugins: Vec<String>,
}

impl Uninstall {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::get_plugins_dir()?;
        for plugin in &self.plugins {
            println!("Uninstalling {}...", plugin);
            std::fs::remove_dir_all(dir.join(plugin))?;
        }
        Ok(())
    }
}

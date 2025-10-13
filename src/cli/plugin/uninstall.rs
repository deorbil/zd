use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Uninstall {
    /// Name of the plugin to be uninstalled
    name: String,
}

impl Uninstall {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::get_plugin_dir(&self.name)?;
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}

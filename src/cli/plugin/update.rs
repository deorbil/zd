use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Update {
    /// Name of the plugin to be updated
    name: String,
}

impl Update {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::get_plugin_dir(&self.name)?;
        utils::git::pull(&dir)?;
        Ok(())
    }
}

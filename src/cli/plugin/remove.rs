use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct Remove {
    name: String,
}

impl Remove {
    pub fn run(&self) -> Result<()> {
        let dir = utils::env::get_plugins_dir()?.join(&self.name);
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}

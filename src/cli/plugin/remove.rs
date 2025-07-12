use anyhow::Result;
use clap::Parser;

use crate::utils::env;

#[derive(Parser)]
pub struct Remove {
    name: String,
}

impl Remove {
    pub fn run(&self) -> Result<()> {
        let dir = env::get_plugins_dir()?.join(&self.name);
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}

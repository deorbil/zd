use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct List;

impl List {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::get_plugins_dir()?;
        std::fs::create_dir_all(&dir)?;

        let entries = std::fs::read_dir(&dir)?;
        for entry in entries.flatten().filter(|entry| entry.path().is_dir()) {
            println!("{}", entry.file_name().to_string_lossy());
        }

        Ok(())
    }
}

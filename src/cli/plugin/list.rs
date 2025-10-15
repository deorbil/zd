use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct List;

impl List {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::create_plugins_dir()?;

        let mut entries = std::fs::read_dir(&dir)?
            .flatten()
            .filter(|entry| entry.path().is_dir())
            .peekable();

        if entries.peek().is_some() {
            for entry in entries {
                println!("{}", entry.file_name().to_string_lossy());
            }
        } else {
            println!("No installed plugins.");
        }

        Ok(())
    }
}

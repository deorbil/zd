use anyhow::Result;
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct List;

impl List {
    pub fn run(&self) -> Result<()> {
        let dir = utils::path::create_plugins_dir()?;

        let entries = utils::fs::get_dirs(&dir)?;
        if !entries.is_empty() {
            for entry in entries {
                println!("{}", entry.file_name().to_string_lossy());
            }
        } else {
            println!("No installed plugins.");
        }

        Ok(())
    }
}

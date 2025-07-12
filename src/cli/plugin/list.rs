use anyhow::Result;
use clap::Parser;

use crate::utils::env;

#[derive(Parser)]
pub struct List;

impl List {
    pub fn run(&self) -> Result<()> {
        let dir = env::get_plugins_dir()?;

        let entries = std::fs::read_dir(&dir)?;
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Ok(name) = entry.file_name().into_string() {
                    println!("{}", name);
                }
            }
        }

        Ok(())
    }
}

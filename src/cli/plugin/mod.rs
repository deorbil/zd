mod add;
mod list;
mod remove;

use anyhow::Result;
use clap::{Parser, Subcommand};

use add::Add;
use list::List;
use remove::Remove;

#[derive(Subcommand)]
pub enum Commands {
    /// Install a plugin
    Add(Add),
    /// List all installed plugins
    #[command(alias = "ls")]
    List(List),
    /// Uninstall a plugin
    #[command(alias = "rm")]
    Remove(Remove),
}

#[derive(Parser)]
pub struct Plugin {
    #[command(subcommand)]
    command: Commands,
}

impl Plugin {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Add(add) => add.run(),
            Commands::List(list) => list.run(),
            Commands::Remove(remove) => remove.run(),
        }
    }
}

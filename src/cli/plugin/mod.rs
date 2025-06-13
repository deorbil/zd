mod add;
mod remove;

use clap::{Parser, Subcommand};

use add::Add;
use remove::Remove;

#[derive(Subcommand)]
pub enum Commands {
    /// Install a plugin
    Add(Add),
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
    pub fn run(&self) {
        match &self.command {
            Commands::Add(add) => add.run(),
            Commands::Remove(remove) => remove.run(),
        }
    }
}

mod init;
mod plugin;

use anyhow::Result;
use clap::{Parser, Subcommand};

use init::Init;
use plugin::Plugin;

#[derive(Subcommand)]
pub enum Commands {
    /// Print the initialization script
    Init(Init),
    /// Manage plugins
    Plugin(Plugin),
}

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Init(init) => init.run(),
            Commands::Plugin(plugin) => plugin.run(),
        }
    }
}

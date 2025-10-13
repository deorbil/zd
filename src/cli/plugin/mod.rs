mod install;
mod list;
mod uninstall;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

use install::Install;
use list::List;
use uninstall::Uninstall;
use update::Update;

#[derive(Subcommand)]
pub enum Commands {
    /// Install a plugin
    #[command(aliases = ["i", "add"])]
    Install(Install),
    /// List installed plugins
    #[command(alias = "ls")]
    List(List),
    /// Uninstall a plugin
    #[command(aliases = ["remove", "rm"])]
    Uninstall(Uninstall),
    /// Update a plugin
    Update(Update),
}

#[derive(Parser)]
pub struct Plugin {
    #[command(subcommand)]
    command: Commands,
}

impl Plugin {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Install(install) => install.run(),
            Commands::List(list) => list.run(),
            Commands::Uninstall(uninstall) => uninstall.run(),
            Commands::Update(update) => update.run(),
        }
    }
}

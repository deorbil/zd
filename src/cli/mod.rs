mod init;

use clap::{Parser, Subcommand};

use init::Init;

#[derive(Subcommand)]
pub enum Commands {
    /// Print shell script
    Init(Init),
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Init(init) => init.run(),
        }
    }
}

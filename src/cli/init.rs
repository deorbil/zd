use anyhow::Result;
use clap::Parser;

use crate::shell::Shell;

#[derive(Parser)]
pub struct Init {
    shell: Shell,
}

impl Init {
    pub fn run(&self) -> Result<()> {
        println!("{}", self.shell.render());
        Ok(())
    }
}

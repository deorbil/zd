use anyhow::Result;
use askama::Template;
use clap::{Parser, ValueEnum};

use crate::templates;

#[derive(ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Parser)]
pub struct Init {
    shell: Shell,
}

impl Init {
    pub fn run(&self) -> Result<()> {
        let content = match &self.shell {
            Shell::Bash => templates::bin::bash::init::Template.render()?,
            Shell::Zsh => templates::bin::zsh::init::Template.render()?,
        };
        println!("{}", content);
        Ok(())
    }
}

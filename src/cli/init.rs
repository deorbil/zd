use askama::Template;
use clap::{Parser, ValueEnum};

use crate::templates;

#[derive(ValueEnum, Clone)]
pub enum Shell {
    Bash,
}

#[derive(Parser)]
pub struct Init {
    shell: Shell,
}

impl Init {
    pub fn run(&self) {
        let content = match &self.shell {
            Shell::Bash => templates::bin::bash::init::Template.render().unwrap(),
        };
        println!("{}", content);
    }
}

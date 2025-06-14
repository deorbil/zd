use askama::Template;
use clap::{Parser, ValueEnum};

use crate::templates::Bash;

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
            Shell::Bash => Bash.render().unwrap(),
        };
        println!("{}", content);
    }
}

mod cli;
mod env;
mod templates;

use clap::Parser;

use cli::Cli;

fn main() {
    Cli::parse().run();
}

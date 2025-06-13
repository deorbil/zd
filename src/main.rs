mod cli;
mod templates;

use clap::Parser;

use cli::Cli;

fn main() {
    Cli::parse().run();
}

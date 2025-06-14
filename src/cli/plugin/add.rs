use clap::Parser;
use git2::Repository;

use crate::env;

#[derive(Parser)]
pub struct Add {
    name: String,
    repository: String,
}

impl Add {
    pub fn run(&self) {
        let dir = env::get_zd_dir().join("plugins").join(&self.name);
        std::fs::create_dir_all(&dir).unwrap();

        Repository::clone(&self.repository, dir).unwrap();
    }
}

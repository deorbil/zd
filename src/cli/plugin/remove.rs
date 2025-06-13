use clap::Parser;

use crate::env;

#[derive(Parser)]
pub struct Remove {
    name: String,
}

impl Remove {
    pub fn run(&self) {
        let dir = env::get_zd_dir().join("plugins").join(&self.name);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

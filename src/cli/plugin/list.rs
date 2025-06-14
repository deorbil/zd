use clap::Parser;

use crate::env;

#[derive(Parser)]
pub struct List;

impl List {
    pub fn run(&self) {
        let dir = env::get_zd_dir().join("plugins");

        let entries = std::fs::read_dir(&dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let name = entry.file_name().into_string().unwrap();
                println!("{}", name);
            }
        }
    }
}

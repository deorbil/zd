use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

impl Shell {
    pub fn render(&self) -> &str {
        match self {
            Shell::Bash => include_str!("zd.bash"),
            Shell::Zsh => include_str!("zd.zsh"),
        }
    }
}

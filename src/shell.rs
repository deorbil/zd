use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

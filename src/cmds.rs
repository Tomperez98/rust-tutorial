use clap::{Args, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum Mode {
    Loud,
    Quiet,
}

#[derive(Args)]
pub struct Greet {
    #[arg(short = 'm')]
    #[arg(value_enum, default_value_t = Mode::Quiet)]
    pub mode: Mode,
    pub name: String,
}

#[derive(Args)]
pub struct GuessNumber {
    pub max_number: u8,
}
#[derive(Subcommand)]
pub enum SubCommands {
    Greet(Greet),
    GuessNumber(GuessNumber),
    Generics,
    Traits,
    Ownership,
    Collections,
}

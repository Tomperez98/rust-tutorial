use clap::{Args, Subcommand, ValueEnum};
#[derive(ValueEnum, Clone, Default)]
pub enum Mode {
    Loud,
    #[default]
    Quiet,
}
#[derive(Args)]
pub struct Greet {
    #[arg(short = 'm')]
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
}

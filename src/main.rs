use std::io;

use clap::{command, Parser};
mod cmds;
use rand::{self, Rng};
#[derive(Parser)]
#[command(name = "Rust Tutorial")]
#[command(author = "Tomas Perez")]
#[command(version = "1.0")]
#[command(about = "Execute demo application writen in... Rust", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    sub_commands: cmds::SubCommands,
}

fn guessing_game(max_number: u8) {
    let secret_number = rand::thread_rng().gen_range(1..max_number);
    println!("Guess the number");

    loop {
        let mut guessed_number = String::new();
        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read the line.");

        let guessed_number: u8 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
        match guessed_number.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small... try again"),
            std::cmp::Ordering::Equal => {
                println!("You won!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too large... try again"),
        }
    }
}

fn execute_greet(name: &str, mode: &cmds::Mode) {
    match mode {
        cmds::Mode::Quiet => println!("hi {}", name.to_lowercase()),
        cmds::Mode::Loud => println!("HI {}", name.to_uppercase()),
    }
}
fn main() {
    let cli = Cli::parse();

    match &cli.sub_commands {
        cmds::SubCommands::Greet(greet) => execute_greet(&greet.name, &greet.mode),
        cmds::SubCommands::GuessNumber(guess_number) => guessing_game(guess_number.max_number),
    }
}

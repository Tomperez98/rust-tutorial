mod base;
mod cmds;
mod jewels_and_stones;
mod two_sum;
use clap::{command, Parser};

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

fn execute_greet(name: &str, mode: &cmds::Mode) {
    let mut greet = "Hi ".to_owned();
    greet.push_str(name);
    match mode {
        cmds::Mode::Quiet => println!("{}", greet.to_lowercase()),
        cmds::Mode::Loud => println!("{}", greet.to_uppercase()),
    }
}
fn main() {
    let cli = Cli::parse();

    match &cli.sub_commands {
        cmds::SubCommands::Greet(greet) => execute_greet(&greet.name, &greet.mode),
        cmds::SubCommands::TwoSum => {
            println!("{:?}", base::Solution::two_sum(vec![2, 7, 11, 15], 9));
            println!("{:?}", base::Solution::two_sum(vec![3, 2, 4], 6));
            println!("{:?}", base::Solution::two_sum(vec![3, 3], 6));
        }
        cmds::SubCommands::JewelsAndStones => {
            println!(
                "{:?}",
                base::Solution::num_jewels_in_stones("aA".to_owned(), "aAAbbbb".to_owned())
            );
            println!(
                "{:?}",
                base::Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned())
            );
        }
    }
}

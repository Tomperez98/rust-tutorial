mod apply_operations;
mod arithmetic_triplets;
mod base;
mod cmds;
mod contains_duplicates;
mod destination_city;
mod index_first_ocurrence;
mod jewels_and_stones;
mod kids_with_candies;
mod length_last_word;
mod move_zeroes;
mod num_of_good_pairs;
mod remove_duplicates_sorted_array;
mod smallest_even_multiple;
mod sort_the_people;
mod top_k_frequent;
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
        cmds::SubCommands::NumGoodPairs => {
            println!(
                "{:?}",
                base::Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3])
            );
            println!(
                "{:?}",
                base::Solution::num_identical_pairs(vec![1, 1, 1, 1])
            );
            println!("{:?}", base::Solution::num_identical_pairs(vec![1, 2, 3]));
        }
        cmds::SubCommands::KidsWithCandies => {
            println!(
                "{:?}",
                base::Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3)
            )
        }
        cmds::SubCommands::ContainsDuplicates => {
            println!("{:?}", base::Solution::contains_duplicate(vec![1, 2, 3, 1]))
        }
        cmds::SubCommands::TopKFrequent => {
            println!(
                "{:?}",
                base::Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
            )
        }
        cmds::SubCommands::RemoveDuplicatesSortedArray => {
            println!(
                "{:?}",
                base::Solution::remove_duplicates(&mut vec![1, 1, 2])
            )
        }
        cmds::SubCommands::ApplyOperations => {
            println!(
                "{:?}",
                base::Solution::apply_operations(vec![1, 2, 2, 1, 1, 0])
            )
        }
        cmds::SubCommands::MoveZeroes => {
            println!(
                "{:?}",
                base::Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12])
            )
        }
        cmds::SubCommands::LengthOfLastWord => {
            println!(
                "{:?}",
                base::Solution::length_of_last_word("luffy is still joyboy".to_owned())
            )
        }
        cmds::SubCommands::DestinationCity => {
            let cities: Vec<Vec<String>> = vec![
                vec!["B".to_owned(), "C".to_owned()],
                vec!["D".to_owned(), "B".to_owned()],
                vec!["C".to_owned(), "A".to_owned()],
            ];
            println!("{:?}", base::Solution::dest_city(cities));
        }
        cmds::SubCommands::IndexFirstOcurrence => {
            let found = base::Solution::str_str("sadbutsad".to_owned(), "sad".to_owned());
            println!("{:?}", found)
        }
        cmds::SubCommands::SmallestEvenMultiple => {
            println!("{:?}", base::Solution::smallest_even_multiple(10))
        }
        cmds::SubCommands::ArithmeticTriplets => {
            println!(
                "{:?}",
                base::Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3)
            )
        }
        cmds::SubCommands::SortThePeople => {
            println!(
                "{:?}",
                base::Solution::sort_people(
                    vec!["Mary".to_owned(), "John".to_owned(), "Emma".to_owned()],
                    vec![180, 165, 170]
                )
            )
        }
    }
}

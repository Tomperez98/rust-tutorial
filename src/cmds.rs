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

#[derive(Subcommand)]
pub enum SubCommands {
    Greet(Greet),
    TwoSum,
    JewelsAndStones,
    NumGoodPairs,
    KidsWithCandies,
    ContainsDuplicates,
    TopKFrequent,
    RemoveDuplicatesSortedArray,
    ApplyOperations,
    MoveZeroes,
    LengthOfLastWord,
    DestinationCity,
    IndexFirstOcurrence,
    SmallestEvenMultiple,
    ArithmeticTriplets,
    SortThePeople,
}

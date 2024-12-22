use std::fs;

use challenge::Challenge;
use challenges::prelude::*;
pub use day::Day;
pub use environment::Environment;

pub mod challenge;
mod challenges;
pub mod day;
mod environment;

pub fn run(day: Day, environment: Environment) {
    let day = *day;

    let day: Box<dyn Challenge> = match day {
        1 => Box::new(ChallengeOne),
        2 => Box::new(ChallengeTwo),
        3 => Box::new(ChallengeThree),
        4 => Box::new(ChallengeFour),
        5 => Box::new(ChallengeFive),
        6 => Box::new(ChallengeSix),
        7 => Box::new(ChallengeSeven),
        8 => Box::new(ChallengeEight),
        9 => Box::new(ChallengeNine),
        10 => Box::new(ChallengeTen),
        11 => Box::new(ChallengeEleven),
        12 => Box::new(ChallengeTwelve),
        13 => Box::new(ChallengeThirteen),
        14 => Box::new(ChallengeFourteen),
        15 => Box::new(ChallengeFifteen),
        17 => Box::new(ChallengeSeventeen),
        18 => Box::new(ChallengeEighteen),
        _ => unimplemented!("Missing solution for challenge {day}"),
    };

    let path = day.path(environment);

    let input = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file {:?}", path.file_name()));

    day.run(input.trim().to_string())
        .iter()
        .for_each(|i| println!("{i}"));
}

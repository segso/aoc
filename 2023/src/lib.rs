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
        _ => unimplemented!("Missing solution for challenge {day}"),
    };

    let path = day.path(environment);

    let input = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file {:?}", path.file_name()));

    day.run(input.trim().to_string())
        .iter()
        .for_each(|i| println!("{i}"));
}

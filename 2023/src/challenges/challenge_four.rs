use std::collections::VecDeque;

use crate::{challenge::Challenge, Day};

pub struct ChallengeFour;

impl Challenge for ChallengeFour {
    fn day(&self) -> Day {
        Day::from(4).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let first_part_result = input
            .lines()
            .map(|line| line[line.find(':').unwrap() + 2..].to_string())
            .map(|line| {
                let correct_numbers = correct_numbers(&line);

                if correct_numbers == 0 {
                    0
                } else {
                    2u32.pow(correct_numbers - 1)
                }
            })
            .sum::<u32>();

        let mut second_part_result = 0;

        let numbers_cache = input
            .lines()
            .map(|line| line[line.find(':').unwrap() + 2..].to_string())
            .map(|line| correct_numbers(&line))
            .collect::<Vec<_>>();

        let mut to_check = VecDeque::new();

        for (_, i) in input.lines().zip(1usize..) {
            to_check.push_back(i);
        }

        while !to_check.is_empty() {
            second_part_result += 1;
            let i = to_check.pop_front().unwrap();
            let correct_numbers = *numbers_cache.get(i - 1).unwrap();

            for i in i + 1..=i + correct_numbers as usize {
                to_check.push_back(i);
            }
        }

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn correct_numbers(line: &str) -> u32 {
    let mut split = line.split('|');

    let winning_numbers = split
        .next()
        .unwrap()
        .split(' ')
        .filter(|num| !num.is_empty())
        .collect::<Vec<_>>();

    let own_numbers = split
        .next()
        .unwrap()
        .split(' ')
        .filter(|num| !num.is_empty())
        .collect::<Vec<_>>();

    winning_numbers.iter().fold(0, |acc, i| {
        if own_numbers.contains(i) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_four() {
        let challenge = ChallengeFour;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec![format!("First part: 13"), format!("Second part: 30")]
        );
    }
}

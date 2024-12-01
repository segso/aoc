use std::collections::HashMap;

use crate::{challenge::Challenge, Day};

pub struct ChallengeOne;

impl Challenge for ChallengeOne {
    fn day(&self) -> crate::Day {
        Day::from(1).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut first_list = Vec::new();
        let mut second_list = Vec::new();

        input
            .lines()
            .map(|line| {
                let mut split = line.split("   ");
                (
                    split.next().unwrap().parse::<u32>().unwrap(),
                    split.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .for_each(|(first_element, second_element)| {
                first_list.push(first_element);
                second_list.push(second_element)
            });

        first_list.sort();
        second_list.sort();

        let sum = first_list
            .iter()
            .zip(second_list.iter())
            .map(|(&first_element, &second_element)| first_element.abs_diff(second_element))
            .sum::<u32>();

        let mut frequencies = HashMap::new();

        for element in second_list {
            *frequencies.entry(element).or_insert(0) += 1;
        }

        let score = first_list
            .iter()
            .map(|&element| element * frequencies.get(&element).unwrap_or(&0))
            .sum::<u32>();

        vec![
            format!("First part: {sum}"),
            format!("Second part: {score}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_one() {
        let challenge = ChallengeOne;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 11", "Second part: 31"]
        );
    }
}

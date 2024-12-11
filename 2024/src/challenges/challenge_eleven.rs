use crate::{challenge::Challenge, Day};
use std::collections::HashMap;

fn solve(cache: &mut HashMap<(u64, usize), usize>, stone: u64, times: usize) -> usize {
    if let Some(result) = cache.get(&(stone, times)) {
        return *result;
    }

    if times == 1 {
        return match stone {
            _ if stone.to_string().len() % 2 == 0 => 2,
            _ => 1,
        };
    }

    match stone {
        0 => {
            let result = solve(cache, 1, times - 1);
            cache.insert((1, times - 1), result);
            result
        }
        _ if stone.to_string().len() % 2 == 0 => {
            let as_string = stone.to_string();
            let half = as_string.len() / 2;
            let first_half = as_string[0..half].parse::<u64>().unwrap();
            let second_half = as_string[half..half * 2].parse::<u64>().unwrap();

            let first = solve(cache, first_half, times - 1);
            cache.insert((first_half, times - 1), first);

            let second = solve(cache, second_half, times - 1);
            cache.insert((second_half, times - 1), second);

            first + second
        }
        _ => {
            let result = solve(cache, stone * 2024, times - 1);
            cache.insert((stone * 2024, times - 1), result);
            result
        }
    }
}

pub struct ChallengeEleven;

impl Challenge for ChallengeEleven {
    fn day(&self) -> crate::Day {
        Day::from(11).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let initial_stones = input
            .trim()
            .split(' ')
            .map(|stone| stone.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut cache = HashMap::new();

        let first_part = initial_stones
            .clone()
            .into_iter()
            .map(|stone| solve(&mut cache, stone, 25))
            .sum::<usize>();

        let second_part = initial_stones
            .into_iter()
            .map(|stone| solve(&mut cache, stone, 75))
            .sum::<usize>();

        vec![
            format!("First part: {first_part}"),
            format!("Second part: {second_part}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_eleven() {
        let challenge = ChallengeEleven;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 55312", "Second part: 65601038650482"]
        );
    }
}

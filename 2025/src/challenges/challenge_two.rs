use crate::{challenge::Challenge, Day};

pub struct ChallengeTwo;

fn has_one_repetition(text: &str) -> bool {
    let midpoint = text.len() / 2;
    let first_half = &text[..midpoint];
    let last_half = &text[midpoint..];

    first_half == last_half
}

fn has_repetitions(text: &str) -> bool {
    let len = text.len();
    'outer: for size in 1..=(len / 2) {
        if !len.is_multiple_of(size) {
            continue;
        }

        let chunk = &text[0..size];

        for index in (0..=(len - size)).step_by(size) {
            if chunk != &text[index..index + size] {
                continue 'outer;
            }
        }

        return true;
    }

    false
}

impl Challenge for ChallengeTwo {
    fn day(&self) -> crate::Day {
        Day::from(2).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let ranges = input
            .trim()
            .split(',')
            .map(|range| {
                let mut split = range.split('-');
                (
                    split.next().unwrap().parse::<i64>().unwrap(),
                    split.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let mut sum = 0;

        for (min, max) in ranges.clone() {
            for i in min..=max {
                if has_one_repetition(&i.to_string()) {
                    sum += i;
                }
            }
        }

        let mut second_sum = 0;

        for (min, max) in ranges {
            for i in min..=max {
                if has_repetitions(&i.to_string()) {
                    second_sum += i;
                }
            }
        }

        vec![
            format!("First part: {sum}"),
            format!("Second part: {second_sum}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_two() {
        let challenge = ChallengeTwo;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 1227775554", "Second part: 4174379265"]
        );
    }
}

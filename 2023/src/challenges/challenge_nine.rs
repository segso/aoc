use crate::{challenge::Challenge, Day};

pub struct ChallengeNine;

impl Challenge for ChallengeNine {
    fn day(&self) -> Day {
        Day::from(9).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let first_part_result = input
            .lines()
            .map(|sequence| {
                let sequence = sequence
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();

                let mut stack = create_stack(sequence);

                stack.last_mut().unwrap().push(0);

                while stack.len() != 1 {
                    let value = *stack.pop().unwrap().last().unwrap();
                    let last_vec = stack.last_mut().unwrap();
                    let last_value = last_vec.last().unwrap();

                    last_vec.push(value + last_value);
                }

                *stack.last().unwrap().last().unwrap()
            })
            .sum::<i64>();

        let second_part_result = input
            .lines()
            .map(|sequence| {
                let sequence = sequence
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();

                let mut stack = create_stack(sequence);

                stack.last_mut().unwrap().push(0);

                while stack.len() != 1 {
                    let value = *stack.pop().unwrap().last().unwrap();
                    let last_vec = stack.last_mut().unwrap();
                    let first_value = last_vec[0];

                    last_vec.push(first_value - value);
                }

                *stack.last().unwrap().last().unwrap()
            })
            .sum::<i64>();

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn create_stack(sequence: Vec<i64>) -> Vec<Vec<i64>> {
    let mut stack = vec![sequence];

    while !stack.last().unwrap().iter().all(|&n| n == 0) {
        let last = stack.last().unwrap();
        let mut new = Vec::new();

        for i in 1..last.len() {
            new.push(last[i] - last[i - 1]);
        }

        stack.push(new);
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_nine() {
        let challenge = ChallengeNine;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec![format!("First part: 114"), format!("Second part: 2")]
        );
    }
}

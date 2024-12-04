use crate::{challenge::Challenge, Day};

pub struct ChallengeThree;

impl Challenge for ChallengeThree {
    fn day(&self) -> crate::Day {
        Day::from(3).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut parts = input.split("mul(");

        let mut sum = 0;

        for part in parts.clone().skip(1) {
            if !part.contains(")") {
                continue;
            }

            let subparts = part.split(")").next().unwrap();
            let mut numbers = subparts.split(",");

            let Some(first_number) = numbers.next() else {
                continue;
            };

            let Ok(first_number) = first_number.parse::<u32>() else {
                continue;
            };

            let Some(second_number) = numbers.next() else {
                continue;
            };

            let Ok(second_number) = second_number.parse::<u32>() else {
                continue;
            };

            sum += first_number * second_number;
        }

        let mut index = parts.next().unwrap().len() + 4;

        let mut parts = parts
            .map(|part| {
                index += 4 + part.len();
                (index - 4 - part.len(), part)
            })
            .collect::<Vec<_>>();

        for i in 0..input.len() {
            if input.get(i..i + 7) == Some("don't()") {
                parts.push((i, "don't()"));
            }

            if input.get(i..i + 4) == Some("do()") {
                parts.push((i, "do()"));
            }
        }

        parts.sort_by_key(|part| part.0);

        let mut should_count = true;
        let mut second_sum = 0;

        for (_, part) in parts {
            if part == "don't()" {
                should_count = false;
                continue;
            } else if part == "do()" {
                should_count = true;
                continue;
            }

            if !should_count {
                continue;
            }

            if !part.contains(")") {
                continue;
            }

            let subparts = part.split(")").next().unwrap();
            let mut numbers = subparts.split(",");

            let Some(first_number) = numbers.next() else {
                continue;
            };

            let Ok(first_number) = first_number.parse::<u32>() else {
                continue;
            };

            let Some(second_number) = numbers.next() else {
                continue;
            };

            let Ok(second_number) = second_number.parse::<u32>() else {
                continue;
            };

            second_sum += first_number * second_number;
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
    fn challenge_three() {
        let challenge = ChallengeThree;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 161", "Second part: 48"]
        );
    }
}

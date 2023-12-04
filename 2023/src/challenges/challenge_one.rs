use crate::{challenge::Challenge, Day};

pub struct ChallengeOne;

impl Challenge for ChallengeOne {
    fn day(&self) -> Day {
        Day::from(1).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let first_part_result = calculate_vibration_value(&input).to_string();

        let input = input.lines().map(to_numbers).collect::<Vec<_>>();
        let second_part_result = calculate_vibration_value(&input.join("\n"));

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn calculate_vibration_value(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter(|value| value.is_ascii_digit());

            let zero_ascii = '0' as u32;
            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            let first = first as u32 - zero_ascii;
            let last = last as u32 - zero_ascii;

            first * 10 + last
        })
        .sum::<u32>()
}

fn to_numbers(line: &str) -> String {
    let mut buffer = String::new();

    for start in 0..line.len() {
        let value = line.chars().nth(start).unwrap();

        if value.is_ascii_digit() {
            buffer.push(value);
            continue;
        }

        for end in start..=line.len() {
            let substring = &line[start..end];

            let num = match substring {
                "one" => '1',
                "two" => '2',
                "three" => '3',
                "four" => '4',
                "five" => '5',
                "six" => '6',
                "seven" => '7',
                "eight" => '8',
                "nine" => '9',
                _ => continue,
            };

            buffer.push(num);
        }
    }

    buffer
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
            vec!["First part: 234", "Second part: 281"]
        );
    }
}

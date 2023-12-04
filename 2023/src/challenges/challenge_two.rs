use crate::{challenge::Challenge, Day};

pub struct ChallengeTwo;

impl Challenge for ChallengeTwo {
    fn day(&self) -> Day {
        Day::from(2).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let first_part_result = input
            .lines()
            .map(|line| line[line.find(':').unwrap() + 2..].to_string())
            .zip(1..)
            .filter(|(line, _)| !is_impossible(line))
            .map(|(_, i)| i)
            .sum::<usize>();

        let second_part_result = input
            .lines()
            .map(|line| line[line.find(':').unwrap() + 2..].to_string())
            .map(|line| {
                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                for set in line.split(';') {
                    for cubes in set.split(',') {
                        let mut cubes = cubes.trim().split(' ');

                        let amount = cubes.next().unwrap().parse::<u32>().unwrap();
                        let color = cubes.next().unwrap();

                        let pointer = match color {
                            "red" => &mut max_red,
                            "green" => &mut max_green,
                            "blue" => &mut max_blue,
                            _ => unreachable!(),
                        };

                        if amount > *pointer {
                            *pointer = amount;
                        }
                    }
                }

                max_red * max_green * max_blue
            })
            .sum::<u32>();

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn is_impossible(line: &str) -> bool {
    for set in line.split(';') {
        for cubes in set.split(',') {
            let mut cubes = cubes.trim().split(' ');

            let amount = cubes.next().unwrap().parse::<u32>().unwrap();
            let color = cubes.next().unwrap();

            let impossible = match color {
                "red" => amount > 12,
                "green" => amount > 13,
                "blue" => amount > 14,
                _ => unreachable!(),
            };

            if impossible {
                return true;
            }
        }
    }

    false
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
            vec![format!("First part: 8"), format!("Second part: 2286")]
        );
    }
}

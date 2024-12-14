use crate::{challenge::Challenge, Day};

fn solve(values: &[i64]) -> Option<(i64, i64)> {
    let (a_x, a_y) = (values[0], values[1]);
    let (b_x, b_y) = (values[2], values[3]);
    let (p_x, p_y) = (values[4], values[5]);

    let ds = a_x * b_y - b_x * a_y;
    let dx = p_x * b_y - p_y * b_x;
    let dy = p_y * a_x - p_x * a_y;

    let x = dx / ds;
    let y = dy / ds;

    if x * ds != dx || y * ds != dy {
        None
    } else {
        Some((x, y))
    }
}

pub struct ChallengeThirteen;

impl Challenge for ChallengeThirteen {
    fn day(&self) -> Day {
        Day::from(13).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut sum = 0;
        let mut second_sum = 0;

        for item in input.trim().split("\n\n") {
            let mut values = Vec::new();

            for line in item.split("\n") {
                values.extend(
                    line.split(':')
                        .nth(1)
                        .unwrap()
                        .split(',')
                        .map(|element| element.trim()[2..].parse::<i64>().unwrap()),
                );
            }

            if let Some((x, y)) = solve(&values) {
                sum += x * 3 + y;
            }

            values[4] += 10_000_000_000_000;
            values[5] += 10_000_000_000_000;

            if let Some((x, y)) = solve(&values) {
                second_sum += x * 3 + y;
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
    fn challenge_thirteen() {
        let challenge = ChallengeThirteen;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 480", "Second part: 875318608908"]
        );
    }
}

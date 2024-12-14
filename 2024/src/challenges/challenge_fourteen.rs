use std::{collections::HashMap, env};

use crate::{challenge::Challenge, Day, Environment};

fn modulo(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}

pub struct ChallengeFourteen;

impl Challenge for ChallengeFourteen {
    fn day(&self) -> Day {
        Day::from(14).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let environment = match env::var("ENVIRONMENT")
            .unwrap_or("LOCAL".to_string())
            .as_str()
        {
            "PRODUCTION" => Environment::Production,
            "LOCAL" => Environment::Local,
            _ => panic!(),
        };

        let width = match environment {
            Environment::Production => 101,
            Environment::Local => 11,
        };
        let height = match environment {
            Environment::Production => 103,
            Environment::Local => 7,
        };

        let mut robots = HashMap::new();

        for robot in input.trim().lines() {
            let mut split = robot.split(' ');
            let mut robot = split.next().unwrap()[2..]
                .split(',')
                .map(|n| n.parse::<i32>().unwrap());
            let mut step = split.next().unwrap()[2..]
                .split(',')
                .map(|n| n.parse::<i32>().unwrap());

            let (robot_x, robot_y) = (robot.next().unwrap(), robot.next().unwrap());
            let (step_x, step_y) = (step.next().unwrap(), step.next().unwrap());

            let x = modulo(robot_x + (100 * step_x), width);
            let y = modulo(robot_y + (100 * step_y), height);

            *robots.entry((x, y)).or_insert(0) += 1;
        }

        let mut product = 1;

        let quadrants = [
            (0, width / 2, 0, height / 2),
            (width / 2 + 1, width, 0, height / 2),
            (0, width / 2, height / 2 + 1, height),
            (width / 2 + 1, width, height / 2 + 1, height),
        ];

        for (start_x, end_x, start_y, end_y) in quadrants {
            let mut quadrant = 0;
            for x in start_x..end_x {
                for y in start_y..end_y {
                    if let Some(quantity) = robots.get(&(x, y)) {
                        quadrant += quantity;
                    }
                }
            }
            product *= quadrant;
        }

        let mut output = vec![format!("First part: {product}")];

        // Hardcoded because it makes no sense to code this
        if let Environment::Production = environment {
            let x_cycle = 39;
            let y_cycle = 99;

            let mut x = 0;

            for current_x in 0.. {
                if height * current_x % width == x_cycle {
                    x = current_x;
                    break;
                }
            }

            let mut y = 0;
            for current_y in 0.. {
                if width * current_y % height == y_cycle {
                    y = current_y;
                    break;
                }
            }

            let result = height * x + width * y;

            output.push(format!("Second part: {result}"))
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_fourteen() {
        let challenge = ChallengeFourteen;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(challenge.run(input), vec!["First part: 12"]);
    }
}

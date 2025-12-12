use crate::{Day, challenge::Challenge};

fn get(grid: &[Vec<char>], x: usize, y: usize) -> char {
    let Some(row) = grid.get(y) else {
        return ' ';
    };

    *row.get(x).unwrap_or(&' ')
}

pub struct ChallengeSix;

impl Challenge for ChallengeSix {
    fn day(&self) -> crate::Day {
        Day::from(6).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = grid[0].len();
        let height = grid.len();

        let mut total = 0;

        for i in 0..width {
            let operation = grid[height - 1][i];
            let mut result = 1;

            for row in grid.iter().take(height - 1) {
                let num = row[i].parse::<u64>().unwrap();

                if operation == "+" {
                    result += num;
                } else {
                    result *= num;
                }
            }

            if operation == "+" {
                result -= 1;
            }

            total += result;
        }

        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = grid[0].len();
        let height = grid.len();

        let mut second_total = 0;
        let mut numbers = vec![];

        for i in (0..width).rev() {
            let mut number = String::new();

            for j in 0..height - 1 {
                number.push(get(&grid, i, j));
            }

            if number.trim().is_empty() {
                continue;
            }

            numbers.push(number.trim().parse::<u64>().unwrap());

            if get(&grid, i, height - 1) == '+' {
                second_total += numbers.iter().sum::<u64>();
                numbers.clear();
            } else if get(&grid, i, height - 1) == '*' {
                second_total += numbers.iter().product::<u64>();
                numbers.clear();
            }
        }

        vec![
            format!("First part: {total}"),
            format!("Second part: {second_total}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_six() {
        let challenge = ChallengeSix;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 4277556", "Second part: 3263827"]
        );
    }
}

use crate::{challenge::Challenge, Day};

fn check(
    grid: &[Vec<usize>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let tile_height = grid[y][x];

    if tile_height == 9 {
        return vec![(x, y)];
    }

    let mut points = Vec::new();

    if x > 0 && grid[y][x - 1] == tile_height + 1 {
        points.extend(check(grid, x - 1, y, width, height));
    }

    if y > 0 && grid[y - 1][x] == tile_height + 1 {
        points.extend(check(grid, x, y - 1, width, height));
    }

    if x < width - 1 && grid[y][x + 1] == tile_height + 1 {
        points.extend(check(grid, x + 1, y, width, height));
    }

    if y < height - 1 && grid[y + 1][x] == tile_height + 1 {
        points.extend(check(grid, x, y + 1, width, height));
    }

    points
}

pub struct ChallengeTen;

impl Challenge for ChallengeTen {
    fn day(&self) -> crate::Day {
        Day::from(10).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (c as u8 - b'0') as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = grid[0].len();

        let mut initial_positions = Vec::new();

        for (y, line) in grid.iter().enumerate() {
            for (x, element) in line.iter().enumerate() {
                if *element == 0 {
                    initial_positions.push((x, y));
                }
            }
        }

        let sum = initial_positions
            .iter()
            .map(|position| {
                let mut end_positions = check(&grid, position.0, position.1, width, height);
                end_positions.sort();
                end_positions.dedup();
                end_positions.len()
            })
            .sum::<usize>();

        let second_sum = initial_positions
            .into_iter()
            .map(|position| check(&grid, position.0, position.1, width, height).len())
            .sum::<usize>();

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
    fn challenge_ten() {
        let challenge = ChallengeTen;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 36", "Second part: 81"]
        );
    }
}

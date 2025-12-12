use std::collections::HashMap;

use crate::{Day, challenge::Challenge};

fn count_timelines(
    grid: &[Vec<char>],
    counts: &HashMap<(usize, usize), u64>,
    x: usize,
    y: usize,
) -> u64 {
    let mut count = 0;

    for y in (0..y - 1).rev() {
        match grid[y][x] {
            'S' => return 1,
            '^' => return count,
            _ => {}
        }

        if x > 0 {
            count += *counts.get(&(x - 1, y)).unwrap_or(&0);
        }
        count += *counts.get(&(x + 1, y)).unwrap_or(&0);
    }

    count
}

pub struct ChallengeSeven;

impl Challenge for ChallengeSeven {
    fn day(&self) -> crate::Day {
        Day::from(7).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = grid[0].len();

        let mut counts = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == '^' {
                    let count = count_timelines(&grid, &counts, x, y);
                    if count > 0 {
                        counts.insert((x, y), count);
                    }
                }
            }
        }

        let mut count = 0;
        for x in 0..width {
            count += count_timelines(&grid, &counts, x, height);
        }

        vec![
            format!("First part: {}", counts.len()),
            format!("Second part: {count}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_seven() {
        let challenge = ChallengeSeven;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 21", "Second part: 40"]
        );
    }
}

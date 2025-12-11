use crate::{challenge::Challenge, Day};

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            '-'
        } else {
            self.grid[y][x]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.grid[y][x] = c;
    }

    pub fn count_around(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        let mut surroundings = vec![];

        if x > 0 {
            surroundings.push((x - 1, y));
        }

        if y > 0 {
            surroundings.push((x, y - 1));
        }

        if x > 0 && y > 0 {
            surroundings.push((x - 1, y - 1));
        }

        if x < self.width - 1 {
            surroundings.push((x + 1, y));
        }

        if y < self.height - 1 {
            surroundings.push((x, y + 1));
        }

        if x < self.width - 1 && y < self.height - 1 {
            surroundings.push((x + 1, y + 1));
        }

        if x < self.width - 1 && y > 0 {
            surroundings.push((x + 1, y - 1));
        }

        if x > 0 && y < self.height - 1 {
            surroundings.push((x - 1, y + 1));
        }

        for (x, y) in surroundings {
            if self.get(x, y) == '@' {
                count += 1;
            }
        }

        count
    }
}

pub struct ChallengeFour;

impl Challenge for ChallengeFour {
    fn day(&self) -> crate::Day {
        Day::from(4).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut grid = Grid::new(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let mut count = 0;

        for x in 0..grid.width {
            for y in 0..grid.height {
                if grid.get(x, y) == '@' && grid.count_around(x, y) < 4 {
                    count += 1;
                }
            }
        }

        let mut removed = 0;
        let mut iteration_removed = 0;

        loop {
            for x in 0..grid.width {
                for y in 0..grid.height {
                    if grid.get(x, y) == '@' && grid.count_around(x, y) < 4 {
                        removed += 1;
                        iteration_removed += 1;
                        grid.set(x, y, '.');
                    }
                }
            }

            if iteration_removed == 0 {
                break;
            } else {
                iteration_removed = 0;
            }
        }

        vec![
            format!("First part: {count}"),
            format!("Second part: {removed}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_four() {
        let challenge = ChallengeFour;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 13", "Second part: 43"]
        );
    }
}

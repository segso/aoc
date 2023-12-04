use crate::{challenge::Challenge, Day};

pub struct ChallengeThree;

impl Challenge for ChallengeThree {
    fn day(&self) -> Day {
        Day::from(3).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let symbols = symbol_coordinates(&grid);

        let mut first_part_result = 0;

        for (x, y) in symbols {
            for (x, y) in adjacent_numbers(&grid, x, y) {
                first_part_result += full_number(&grid, x, y);
            }
        }

        let second_part_result = symbol_coordinates(&grid)
            .into_iter()
            .filter(|&(x, y)| grid[y][x] == '*')
            .filter_map(|(x, y)| {
                let numbers = adjacent_numbers(&grid, x, y);

                if numbers.len() == 2 {
                    Some(numbers)
                } else {
                    None
                }
            })
            .map(|vec| {
                let mut iter = vec.into_iter();
                let (first_x, first_y) = iter.next().unwrap();
                let (second_x, second_y) = iter.next().unwrap();

                full_number(&grid, first_x, first_y) * full_number(&grid, second_x, second_y)
            })
            .sum::<u32>();

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn symbol_coordinates(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if is_symbol(*character) {
                coordinates.push((x, y));
            }
        }
    }

    coordinates
}

fn is_symbol(value: char) -> bool {
    !value.is_ascii_digit() && value != '.'
}

fn adjacent_numbers(grid: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    let mut positions = Vec::new();

    if y > 0 {
        if grid[y - 1][x].is_ascii_digit() {
            positions.push((x, y - 1));
        } else {
            if x > 0 && grid[y - 1][x - 1].is_ascii_digit() {
                positions.push((x - 1, y - 1));
            }
            if x != width && grid[y - 1][x + 1].is_ascii_digit() {
                positions.push((x + 1, y - 1));
            }
        }
    }

    if x > 0 && grid[y][x - 1].is_ascii_digit() {
        positions.push((x - 1, y));
    }

    if x != width && grid[y][x + 1].is_ascii_digit() {
        positions.push((x + 1, y));
    }

    if y != height {
        if grid[y + 1][x].is_ascii_digit() {
            positions.push((x, y + 1));
        } else {
            if x > 0 && grid[y + 1][x - 1].is_ascii_digit() {
                positions.push((x - 1, y + 1));
            }
            if x != width && grid[y + 1][x + 1].is_ascii_digit() {
                positions.push((x + 1, y + 1));
            }
        }
    }

    positions
}

fn full_number(grid: &[Vec<char>], x: usize, y: usize) -> u32 {
    let mut depth = 1;

    let num = grid[y]
        .iter()
        .take(x)
        .rev()
        .take_while(|value| value.is_ascii_digit())
        .map(|&value| value as u32 - '0' as u32)
        .fold(grid[y][x] as u32 - '0' as u32, |acc, i| {
            let new = acc + i * (10u32.pow(depth));
            depth += 1;
            new
        });

    grid[y]
        .iter()
        .skip(x + 1)
        .take_while(|value| value.is_ascii_digit())
        .map(|&value| value as u32 - '0' as u32)
        .fold(num, |acc, i| acc * 10 + i)
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
            vec![format!("First part: 4361"), format!("Second part: 467835")]
        );
    }
}

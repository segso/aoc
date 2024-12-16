use crate::{challenge::Challenge, Day};

fn can_advance_up(grid: &Vec<Vec<char>>, current_x: usize, current_y: usize) -> bool {
    match grid[current_y - 1][current_x] {
        '#' => false,
        '.' => true,
        'O' => can_advance_up(grid, current_x, current_y - 1),
        '[' => {
            can_advance_up(grid, current_x, current_y - 1)
                && can_advance_up(grid, current_x + 1, current_y - 1)
        }
        _ => {
            can_advance_up(grid, current_x, current_y - 1)
                && can_advance_up(grid, current_x - 1, current_y - 1)
        }
    }
}

fn advance_up(grid: &mut Vec<Vec<char>>, current_x: &mut usize, current_y: &mut usize) {
    if !can_advance_up(grid, *current_x, *current_y) {
        return;
    }

    match grid[*current_y - 1][*current_x] {
        '.' => {
            let current = grid[*current_y][*current_x];
            grid[*current_y][*current_x] = '.';
            grid[*current_y - 1][*current_x] = current;
            *current_y -= 1;
        }
        'O' => {
            advance_up(grid, current_x, &mut (*current_y - 1));
            advance_up(grid, current_x, current_y);
        }
        ']' => {
            advance_up(grid, &mut *current_x, &mut (*current_y - 1));
            advance_up(grid, &mut (*current_x - 1), &mut (*current_y - 1));
            advance_up(grid, &mut *current_x, &mut *current_y);
        }
        _ => {
            advance_up(grid, &mut *current_x, &mut (*current_y - 1));
            advance_up(grid, &mut (*current_x + 1), &mut (*current_y - 1));
            advance_up(grid, &mut *current_x, &mut *current_y);
        }
    }
}

fn can_advance_down(grid: &Vec<Vec<char>>, current_x: usize, current_y: usize) -> bool {
    match grid[current_y + 1][current_x] {
        '#' => false,
        '.' => true,
        'O' => can_advance_down(grid, current_x, current_y + 1),
        '[' => {
            can_advance_down(grid, current_x, current_y + 1)
                && can_advance_down(grid, current_x + 1, current_y + 1)
        }
        _ => {
            can_advance_down(grid, current_x, current_y + 1)
                && can_advance_down(grid, current_x - 1, current_y + 1)
        }
    }
}

fn advance_down(grid: &mut Vec<Vec<char>>, current_x: &mut usize, current_y: &mut usize) {
    if !can_advance_down(grid, *current_x, *current_y) {
        return;
    }

    match grid[*current_y + 1][*current_x] {
        '.' => {
            let current = grid[*current_y][*current_x];
            grid[*current_y][*current_x] = '.';
            grid[*current_y + 1][*current_x] = current;
            *current_y += 1;
        }
        'O' => {
            advance_down(grid, current_x, &mut (*current_y + 1));
            advance_down(grid, current_x, current_y);
        }
        ']' => {
            advance_down(grid, &mut *current_x, &mut (*current_y + 1));
            advance_down(grid, &mut (*current_x - 1), &mut (*current_y + 1));
            advance_down(grid, &mut *current_x, &mut *current_y);
        }
        _ => {
            advance_down(grid, &mut *current_x, &mut (*current_y + 1));
            advance_down(grid, &mut (*current_x + 1), &mut (*current_y + 1));
            advance_down(grid, &mut *current_x, &mut *current_y);
        }
    }
}

fn advance_left(grid: &mut Vec<Vec<char>>, current_x: &mut usize, current_y: &mut usize) {
    if grid[*current_y][*current_x - 1] == '#' {
        return;
    }

    if grid[*current_y][*current_x - 1] != '.' {
        advance_left(grid, &mut (*current_x - 1), &mut *current_y);
    }

    if grid[*current_y][*current_x - 1] == '.' {
        let current = grid[*current_y][*current_x];
        grid[*current_y][*current_x] = '.';
        grid[*current_y][*current_x - 1] = current;
        *current_x -= 1;
    }
}

fn advance_right(grid: &mut Vec<Vec<char>>, current_x: &mut usize, current_y: &mut usize) {
    if grid[*current_y][*current_x + 1] == '#' {
        return;
    }

    if grid[*current_y][*current_x + 1] != '.' {
        advance_right(grid, &mut (*current_x + 1), &mut *current_y);
    }

    if grid[*current_y][*current_x + 1] == '.' {
        let current = grid[*current_y][*current_x];
        grid[*current_y][*current_x] = '.';
        grid[*current_y][*current_x + 1] = current;
        *current_x += 1;
    }
}

fn execute(mut grid: Vec<Vec<char>>, instructions: &str, box_tile: char) -> usize {
    let mut current_x = 0;
    let mut current_y = 0;

    'outer: for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile == '@' {
                current_x = x;
                current_y = y;
                break 'outer;
            }
        }
    }

    for instruction in instructions.chars() {
        match instruction {
            '^' => advance_up(&mut grid, &mut current_x, &mut current_y),
            'v' => advance_down(&mut grid, &mut current_x, &mut current_y),
            '<' => advance_left(&mut grid, &mut current_x, &mut current_y),
            '>' => advance_right(&mut grid, &mut current_x, &mut current_y),
            _ => {}
        }
    }

    let mut sum = 0;

    for (y, item) in grid.iter().enumerate() {
        for (x, tile) in item.iter().enumerate() {
            if *tile == box_tile {
                sum += 100 * y + x;
            }
        }
    }

    sum
}

pub struct ChallengeFifteen;

impl Challenge for ChallengeFifteen {
    fn day(&self) -> Day {
        Day::from(15).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut split = input.split("\n\n");
        let initial_grid = split
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let instructions = split.next().unwrap().lines().collect::<String>();

        let sum = execute(initial_grid.clone(), &instructions, 'O');

        let mut grid = Vec::new();

        for (y, line) in initial_grid.iter().enumerate() {
            grid.push(Vec::new());
            for tile in line {
                let (left, right) = match tile {
                    '#' => ('#', '#'),
                    'O' => ('[', ']'),
                    '.' => ('.', '.'),
                    _ => ('@', '.'),
                };

                grid[y].push(left);
                grid[y].push(right);
            }
        }

        let score = execute(grid, &instructions, '[');

        vec![
            format!("First part: {sum}"),
            format!("Second part: {score}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_fifteen() {
        let challenge = ChallengeFifteen;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 10092", "Second part: 9021"]
        );
    }
}

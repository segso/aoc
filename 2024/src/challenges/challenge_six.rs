use crate::{challenge::Challenge, Day};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

fn advance(
    obstructions: &[(i32, i32)],
    mut direction: Direction,
    x: i32,
    y: i32,
) -> (i32, i32, Direction) {
    loop {
        let offset = direction.offset();
        let next_x = x + offset.0;
        let next_y = y + offset.1;

        if obstructions.contains(&(next_x, next_y)) {
            direction.turn_right();
        } else {
            return (next_x, next_y, direction);
        }
    }
}

fn look_up(obstructions: &[(i32, i32)], x: i32, y: i32) -> Option<(i32, i32)> {
    for lookup_y in (0..y).rev() {
        if obstructions.contains(&(x, lookup_y)) {
            return Some((x, lookup_y));
        }
    }

    None
}

fn look_down(obstructions: &[(i32, i32)], x: i32, y: i32, height: i32) -> Option<(i32, i32)> {
    for lookup_y in y + 1..height {
        if obstructions.contains(&(x, lookup_y)) {
            return Some((x, lookup_y));
        }
    }

    None
}

fn look_left(obstructions: &[(i32, i32)], x: i32, y: i32) -> Option<(i32, i32)> {
    for lookup_x in (0..x).rev() {
        if obstructions.contains(&(lookup_x, y)) {
            return Some((lookup_x, y));
        }
    }

    None
}

fn look_right(obstructions: &[(i32, i32)], x: i32, y: i32, width: i32) -> Option<(i32, i32)> {
    for lookup_x in x + 1..width {
        if obstructions.contains(&(lookup_x, y)) {
            return Some((lookup_x, y));
        }
    }

    None
}

fn has_loop(
    obstructions: &[(i32, i32)],
    mut direction: Direction,
    mut x: i32,
    mut y: i32,
    width: i32,
    height: i32,
) -> bool {
    let mut collisions = Vec::new();

    loop {
        let Some(new_obstruction) = (match direction {
            Direction::Up => look_up(obstructions, x, y),
            Direction::Down => look_down(obstructions, x, y, height),
            Direction::Left => look_left(obstructions, x, y),
            Direction::Right => look_right(obstructions, x, y, width),
        }) else {
            return false;
        };

        if collisions.contains(&(new_obstruction.0, new_obstruction.1, direction)) {
            return true;
        }

        collisions.push((new_obstruction.0, new_obstruction.1, direction));
        let offset = direction.offset();
        x = new_obstruction.0 - offset.0;
        y = new_obstruction.1 - offset.1;
        direction.turn_right();
    }
}

pub struct ChallengeSix;

impl Challenge for ChallengeSix {
    fn day(&self) -> crate::Day {
        Day::from(6).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let mut direction = Direction::Up;
        let mut initial_guard_x = 0;
        let mut initial_guard_y = 0;

        let mut guard_x = 0;
        let mut guard_y = 0;

        let mut obstructions = Vec::new();

        for (y, line) in grid.iter().enumerate() {
            for (x, element) in line.iter().enumerate() {
                if *element == '^' {
                    initial_guard_x = x as i32;
                    initial_guard_y = y as i32;

                    guard_x = x as i32;
                    guard_y = y as i32;
                }

                if *element == '#' {
                    obstructions.push((x as i32, y as i32));
                }
            }
        }

        let mut visited_positions = vec![(guard_x, guard_y)];

        loop {
            let (next_x, next_y, new_direction) =
                advance(&obstructions, direction, guard_x, guard_y);
            direction = new_direction;

            if !(0..width).contains(&next_x) || !(0..height).contains(&next_y) {
                break;
            }

            (guard_x, guard_y) = (next_x, next_y);
            visited_positions.push((guard_x, guard_y));
        }

        visited_positions.sort();
        visited_positions.dedup();

        let first_part = visited_positions.len();

        guard_x = initial_guard_x;
        guard_y = initial_guard_y;
        direction = Direction::Up;

        let mut sum = 0;
        let mut visited_positions = vec![(guard_x, guard_y)];

        loop {
            let (next_x, next_y, new_direction) =
                advance(&obstructions, direction, guard_x, guard_y);

            if !(0..width).contains(&next_x) || !(0..height).contains(&next_y) {
                break;
            }

            if visited_positions.contains(&(next_x, next_y)) {
                guard_x = next_x;
                guard_y = next_y;
                direction = new_direction;
                continue;
            }

            obstructions.push((next_x, next_y));

            if has_loop(&obstructions, direction, guard_x, guard_y, width, height) {
                sum += 1;
            }

            obstructions.pop();

            visited_positions.push((next_x, next_y));
            guard_x = next_x;
            guard_y = next_y;
            direction = new_direction;
        }

        vec![
            format!("First part: {first_part}"),
            format!("Second part: {sum}"),
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
            vec!["First part: 41", "Second part: 6"]
        );
    }
}

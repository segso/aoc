use crate::{challenge::Challenge, Day};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    fn apply(&self, mut x: usize, mut y: usize) -> (usize, usize) {
        match self {
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
        }

        (x, y)
    }
}

fn get_start_connections(x: usize, y: usize, map: &[Vec<char>]) -> Vec<Direction> {
    let mut directions = Vec::new();

    if x > 0 {
        let left = map[y][x - 1];
        if left == 'L' || left == 'F' || left == '-' {
            directions.push(Direction::Left);
        }
    }

    if y > 0 {
        let up = map[y - 1][x];
        if up == '7' || up == 'F' || up == '|' {
            directions.push(Direction::Up);
        }
    }

    let right = *map[y].get(x + 1).unwrap_or(&'.');
    if right == '7' || right == 'J' || right == '-' {
        directions.push(Direction::Right);
    }

    let down = *map.get(y + 1).unwrap_or(&Vec::new()).get(x).unwrap_or(&'.');
    if down == 'J' || down == 'L' || down == '|' {
        directions.push(Direction::Down);
    }

    directions
}

fn get_start_pipe(directions: &[Direction]) -> char {
    let directions = [
        directions.to_vec(),
        directions.iter().map(Clone::clone).rev().collect(),
    ];

    for directions in &directions {
        let pipe = match directions[..] {
            [Direction::Down, Direction::Right] => Some('F'),
            [Direction::Down, Direction::Left] => Some('7'),
            [Direction::Left, Direction::Up] => Some('J'),
            [Direction::Up, Direction::Down] => Some('|'),
            [Direction::Up, Direction::Right] => Some('L'),
            [Direction::Left, Direction::Right] => Some('-'),
            _ => None,
        };

        if let Some(pipe) = pipe {
            return pipe;
        }
    }

    unimplemented!("{directions:?}")
}

fn get_connections(pipe: char) -> Vec<Direction> {
    match pipe {
        'F' => vec![Direction::Down, Direction::Right],
        '7' => vec![Direction::Down, Direction::Left],
        'J' => vec![Direction::Left, Direction::Up],
        '|' => vec![Direction::Up, Direction::Down],
        'L' => vec![Direction::Up, Direction::Right],
        '-' => vec![Direction::Left, Direction::Right],
        _ => unimplemented!("{pipe}"),
    }
}

fn is_inside(x: usize, y: usize, visited: &[(usize, usize)], map: &[Vec<char>]) -> bool {
    if visited.contains(&(x, y)) {
        return false;
    }

    let mut count = 0;
    let mut j_cached = false;
    let mut seven_cached = false;

    for x in (0..x).rev() {
        let pipe = map[y][x];
        if visited.contains(&(x, y)) {
            let is_vertical_border = match pipe {
                '|' => true,
                'J' => {
                    j_cached = true;
                    false
                }
                '7' => {
                    seven_cached = true;
                    false
                }
                'F' => {
                    if seven_cached {
                        seven_cached = false;
                        false
                    } else if j_cached {
                        j_cached = false;
                        true
                    } else {
                        false
                    }
                }
                'L' => {
                    if seven_cached {
                        seven_cached = false;
                        true
                    } else {
                        j_cached = false;
                        false
                    }
                }
                _ => false,
            };
            if is_vertical_border {
                count += 1;
            }
        }
    }

    count % 2 != 0
}

pub struct ChallengeTen;

impl Challenge for ChallengeTen {
    fn day(&self) -> Day {
        Day::from(10).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut start_x = 0;
        let mut start_y = 0;

        'out: for (y, row) in map.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value == 'S' {
                    start_y = y;
                    start_x = x;
                    break 'out;
                }
            }
        }

        let mut connections = get_start_connections(start_x, start_y, &map);
        let start_pipe = get_start_pipe(&connections);
        map[start_y][start_x] = start_pipe;
        let mut connection = connections.remove(0);
        let (mut current_x, mut current_y) = connection.apply(start_x, start_y);
        let mut count = 0;
        let mut visited = vec![(start_x, start_y), (current_x, current_y)];

        while current_x != start_x || current_y != start_y {
            count += 1;
            let mut connections = get_connections(map[current_y][current_x]);

            if connections[0] == connection.opposite() {
                connection = connections.remove(1);
            } else {
                connection = connections.remove(0);
            }
            (current_x, current_y) = connection.apply(current_x, current_y);
            visited.push((current_x, current_y));
        }

        let first_part_result = (count + 1) / 2;

        let mut count = 0;

        for x in 0..map[0].len() {
            for y in 0..map.len() {
                let inside = is_inside(x, y, &visited, &map);
                if inside {
                    count += 1;
                }
            }
        }

        let second_part_result = count;

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
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
            vec![format!("First part: 80"), format!("Second part: 10")]
        );
    }
}

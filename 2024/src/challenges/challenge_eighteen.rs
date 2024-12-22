use std::{collections::HashMap, env};

use crate::{challenge::Challenge, Day, Environment};

fn travel(grid: &[Vec<char>], width: usize, height: usize) -> u64 {
    let mut unvisited = Vec::new();
    let mut minimum_distances = HashMap::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile != '#' {
                unvisited.push((x, y));
                minimum_distances.insert((x, y), u64::MAX);
            }
        }
    }

    minimum_distances.insert((0, 0), 0);

    while !unvisited.is_empty() {
        let (&(x, y), &distance) = unvisited
            .iter()
            .map(|position| (position, minimum_distances.get(position).unwrap()))
            .min_by_key(|(_, distance)| **distance)
            .unwrap();

        unvisited = unvisited
            .into_iter()
            .filter(|position| *position != (x, y))
            .collect::<Vec<_>>();

        if distance == u64::MAX {
            continue;
        }

        let neighbours = [
            (x != 0, (x.wrapping_sub(1), y)),
            (x != width - 1, (x.wrapping_add(1), y)),
            (y != 0, (x, y.wrapping_sub(1))),
            (y != height - 1, (x, y.wrapping_add(1))),
        ]
        .into_iter()
        .filter(|(condition, _)| *condition)
        .map(|(_, neighbour)| neighbour);

        for neighbour in neighbours {
            let Some(&previous_distance) = minimum_distances.get(&neighbour) else {
                continue;
            };
            let new_distance = distance + 1;

            if previous_distance > new_distance {
                minimum_distances.insert(neighbour, new_distance);
            }
        }
    }

    *minimum_distances.get(&(width - 1, height - 1)).unwrap()
}

fn fall_n_bytes(n: usize, input: &str, width: usize, height: usize) -> Vec<Vec<char>> {
    let mut grid = (0..height)
        .map(|_| (0..width).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, line) in input.lines().enumerate() {
        if i == n {
            break;
        }

        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();

        grid[y][x] = '#';
    }

    grid
}

pub struct ChallengeEighteen;

impl Challenge for ChallengeEighteen {
    fn day(&self) -> Day {
        Day::from(18).unwrap()
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

        let (width, height, bytes) = match environment {
            Environment::Production => (71, 71, 1024),
            Environment::Local => (7, 7, 12),
        };

        let grid = fall_n_bytes(bytes, &input, width, height);

        let distance = travel(&grid, width, height);

        let mut left_bound = bytes;
        let mut right_bound = input.lines().count();

        loop {
            if left_bound == right_bound - 1 {
                break;
            }

            let middle = left_bound + (right_bound - left_bound) / 2;
            let grid = fall_n_bytes(middle, &input, width, height);
            let distance = travel(&grid, width, height);

            if distance == u64::MAX {
                right_bound = middle;
            } else {
                left_bound = middle;
            }
        }

        vec![
            format!("First part: {distance}"),
            format!("Second part: {}", input.lines().nth(left_bound).unwrap()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_eighteen() {
        let challenge = ChallengeEighteen;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 22", "Second part: 6,1"]
        );
    }
}

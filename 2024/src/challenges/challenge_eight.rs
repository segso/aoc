use std::collections::HashMap;

use crate::{challenge::Challenge, Day};

fn get_two_antinodes(antennas: [(i32, i32); 2]) -> [(i32, i32); 2] {
    let (first_x, first_y) = (antennas[0].0, antennas[0].1);
    let (second_x, second_y) = (antennas[1].0, antennas[1].1);

    [
        (2 * first_x - second_x, 2 * first_y - second_y),
        (2 * second_x - first_x, 2 * second_y - first_y),
    ]
}

fn get_antinodes(antennas: [(i32, i32); 2], width: i32, height: i32) -> Vec<(i32, i32)> {
    let mut antinodes = vec![antennas[0], antennas[1]];

    let x_change = antennas[1].0 - antennas[0].0;
    let y_change = antennas[1].1 - antennas[0].1;

    let mut current_x = antennas[0].0;
    let mut current_y = antennas[0].1;

    while (0..width).contains(&current_x) && (0..height).contains(&current_y) {
        current_x -= x_change;
        current_y -= y_change;

        antinodes.push((current_x, current_y));
    }

    antinodes.pop();

    let mut current_x = antennas[1].0;
    let mut current_y = antennas[1].1;

    while (0..width).contains(&current_x) && (0..height).contains(&current_y) {
        current_x += x_change;
        current_y += y_change;

        antinodes.push((current_x, current_y));
    }

    antinodes.pop();

    antinodes
}

pub struct ChallengeEight;

impl Challenge for ChallengeEight {
    fn day(&self) -> crate::Day {
        Day::from(8).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let mut antennas = HashMap::new();

        for (y, line) in grid.iter().enumerate() {
            for (x, element) in line.iter().enumerate() {
                if element.is_ascii_alphabetic() || element.is_numeric() {
                    antennas
                        .entry(element)
                        .or_insert(Vec::new())
                        .push((x as i32, y as i32));
                }
            }
        }

        let mut antinodes = Vec::new();

        for frequency in antennas.keys() {
            let antennas = antennas.get(frequency).unwrap();

            for i in 0..antennas.len() - 1 {
                for j in i + 1..antennas.len() {
                    antinodes.extend(get_two_antinodes([antennas[i], antennas[j]]));
                }
            }
        }

        antinodes.sort();
        antinodes.dedup();

        let antinodes = antinodes
            .into_iter()
            .filter(|(x, y)| (0..width).contains(x) && (0..height).contains(y))
            .collect::<Vec<_>>();

        let first_part = antinodes.len();

        let mut antinodes = Vec::new();

        for frequency in antennas.keys() {
            let antennas = antennas.get(frequency).unwrap();

            for i in 0..antennas.len() - 1 {
                for j in i + 1..antennas.len() {
                    antinodes.extend(get_antinodes([antennas[i], antennas[j]], width, height));
                }
            }
        }

        antinodes.sort();
        antinodes.dedup();

        vec![
            format!("First part: {first_part}"),
            format!("Second part: {}", antinodes.len()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_eight() {
        let challenge = ChallengeEight;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 14", "Second part: 34"]
        );
    }
}

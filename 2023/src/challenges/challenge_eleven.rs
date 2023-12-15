use crate::{challenge::Challenge, Day};

fn calculate_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> u64 {
    let x1 = x1 as u64;
    let y1 = y1 as u64;
    let x2 = x2 as u64;
    let y2 = y2 as u64;

    x2.abs_diff(x1) + y2.abs_diff(y1)
}

fn calculate_distance_sum(map: &[Vec<char>], increment: usize) -> u64 {
    let mut galaxies = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let empty_rows = map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(y, _)| y)
        .rev();

    for y in empty_rows {
        galaxies
            .iter_mut()
            .filter(|(_, gy)| *gy > y)
            .for_each(|(_, gy)| *gy += increment);
    }

    let empty_columns = (0..map[0].len())
        .filter(|&x| map.iter().all(|row| row[x] == '.'))
        .rev();

    for x in empty_columns {
        galaxies
            .iter_mut()
            .filter(|(gx, _)| *gx > x)
            .for_each(|(gx, _)| *gx += increment);
    }

    let mut count = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            count += calculate_distance(galaxies[i], galaxies[j]);
        }
    }

    count
}

pub struct ChallengeEleven;

impl Challenge for ChallengeEleven {
    fn day(&self) -> Day {
        Day::from(11).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let first_part_result = calculate_distance_sum(&map, 1);
        let second_part_result = calculate_distance_sum(&map, 999999);

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
    fn challenge_eleven() {
        let challenge = ChallengeEleven;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec![format!("First part: 374"), format!("Second part: 82000210")]
        );
    }
}

use crate::{challenge::Challenge, Day};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq)]
struct Fence {
    direction: Direction,
    x: usize,
    y: usize,
}

impl Fence {
    fn new(direction: Direction, x: usize, y: usize) -> Self {
        Self { direction, x, y }
    }
}

fn look_region(
    grid: &[&[char]],
    visited: &mut Vec<(usize, usize)>,
    fences: &mut Vec<Fence>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> (usize, usize) {
    let plant = grid[y][x];

    let mut area = 1;
    let mut perimeter = 0;

    visited.push((x, y));

    if x == 0 || grid[y][x - 1] != plant {
        perimeter += 1;
        fences.push(Fence::new(Direction::Left, x, y));
    } else if !visited.contains(&(x - 1, y)) {
        let region = look_region(grid, visited, fences, x - 1, y, width, height);
        area += region.0;
        perimeter += region.1;
    }

    if x == width - 1 || grid[y][x + 1] != plant {
        perimeter += 1;
        fences.push(Fence::new(Direction::Right, x, y));
    } else if !visited.contains(&(x + 1, y)) {
        let region = look_region(grid, visited, fences, x + 1, y, width, height);
        area += region.0;
        perimeter += region.1;
    }

    if y == 0 || grid[y - 1][x] != plant {
        perimeter += 1;
        fences.push(Fence::new(Direction::Up, x, y));
    } else if !visited.contains(&(x, y - 1)) {
        let region = look_region(grid, visited, fences, x, y - 1, width, height);
        area += region.0;
        perimeter += region.1;
    }

    if y == height - 1 || grid[y + 1][x] != plant {
        perimeter += 1;
        fences.push(Fence::new(Direction::Down, x, y));
    } else if !visited.contains(&(x, y + 1)) {
        let region = look_region(grid, visited, fences, x, y + 1, width, height);
        area += region.0;
        perimeter += region.1;
        visited.push((x, y + 1));
    }

    (area, perimeter)
}

pub struct ChallengeTwelve;

impl Challenge for ChallengeTwelve {
    fn day(&self) -> crate::Day {
        Day::from(12).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = grid.len();
        let width = grid[0].len();

        let mut visited = Vec::new();
        let mut regions = Vec::new();
        let mut sum = 0;

        for (y, line) in grid.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                if visited.contains(&(x, y)) {
                    continue;
                }

                let mut fences = Vec::new();

                let (area, perimeter) = look_region(
                    &grid.iter().map(Vec::as_slice).collect::<Vec<_>>(),
                    &mut visited,
                    &mut fences,
                    x,
                    y,
                    width,
                    height,
                );

                sum += area * perimeter;
                regions.push((area, fences));
            }
        }

        for (_area, region_fences) in &mut regions {
            let mut i = 0;

            loop {
                if i >= region_fences.len() {
                    break;
                }

                let fence = &mut region_fences[i];
                let direction = fence.direction;
                let x = fence.x;
                let y = fence.y;

                for x in x + 1..width {
                    if let Some(j) = region_fences
                        .iter()
                        .enumerate()
                        .find(|(_, fence)| fence == &&Fence { direction, x, y })
                        .map(|(i, _)| i)
                    {
                        region_fences.remove(j);
                        i = i.saturating_sub(1);
                    } else {
                        break;
                    }
                }

                for y in y + 1..height {
                    if let Some(j) = region_fences
                        .iter()
                        .enumerate()
                        .find(|(_, fence)| fence == &&Fence { direction, x, y })
                        .map(|(i, _)| i)
                    {
                        region_fences.remove(j);
                        i = i.saturating_sub(1);
                    } else {
                        break;
                    }
                }

                i += 1;
            }
        }

        let mut second_sum = 0;

        for region in regions {
            second_sum += region.0 * region.1.len();
        }

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
    fn challenge_twelve() {
        let challenge = ChallengeTwelve;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 1930", "Second part: 1206"]
        );
    }
}

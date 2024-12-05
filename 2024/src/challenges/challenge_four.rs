use crate::{challenge::Challenge, Day};

fn check_xmas(first: char, second: char, third: char, fourth: char) -> bool {
    first == 'X' && second == 'M' && third == 'A' && fourth == 'S'
}

fn check_cross_mas(input: &[Vec<char>], x: usize, y: usize, width: usize, height: usize) -> bool {
    if x < 1 || x > width - 2 || y < 1 || y > height - 2 {
        return false;
    }

    let first = (input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S')
        || (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M');

    let second = (input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S')
        || (input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M');

    first && second
}

fn look_up(input: &[Vec<char>], x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y - 1][x],
        input[y - 2][x],
        input[y - 3][x],
    )
}

fn look_down(input: &[Vec<char>], x: usize, y: usize, height: usize) -> bool {
    if y > height - 4 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y + 1][x],
        input[y + 2][x],
        input[y + 3][x],
    )
}

fn look_left(input: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y][x - 1],
        input[y][x - 2],
        input[y][x - 3],
    )
}

fn look_right(input: &[Vec<char>], x: usize, y: usize, width: usize) -> bool {
    if x > width - 4 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y][x + 1],
        input[y][x + 2],
        input[y][x + 3],
    )
}

fn look_up_left(input: &[Vec<char>], x: usize, y: usize) -> bool {
    if y < 3 || x < 3 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y - 1][x - 1],
        input[y - 2][x - 2],
        input[y - 3][x - 3],
    )
}

fn look_up_right(input: &[Vec<char>], x: usize, y: usize, width: usize) -> bool {
    if y < 3 || x > width - 4 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y - 1][x + 1],
        input[y - 2][x + 2],
        input[y - 3][x + 3],
    )
}

fn look_down_right(input: &[Vec<char>], x: usize, y: usize, width: usize, height: usize) -> bool {
    if y > height - 4 || x > width - 4 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y + 1][x + 1],
        input[y + 2][x + 2],
        input[y + 3][x + 3],
    )
}

fn look_down_left(input: &[Vec<char>], x: usize, y: usize, height: usize) -> bool {
    if y > height - 4 || x < 3 {
        return false;
    }

    check_xmas(
        input[y][x],
        input[y + 1][x - 1],
        input[y + 2][x - 2],
        input[y + 3][x - 3],
    )
}

pub struct ChallengeFour;

impl Challenge for ChallengeFour {
    fn day(&self) -> crate::Day {
        Day::from(4).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let input = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = input.len();
        let width = input[0].len();
        let mut sum = 0;

        for (y, line) in input.iter().enumerate() {
            for (x, letter) in line.iter().enumerate() {
                if *letter != 'X' {
                    continue;
                }

                if look_up(&input, x, y) {
                    sum += 1;
                }

                if look_down(&input, x, y, height) {
                    sum += 1;
                }

                if look_left(&input, x, y) {
                    sum += 1;
                }

                if look_right(&input, x, y, width) {
                    sum += 1;
                }

                if look_up_left(&input, x, y) {
                    sum += 1;
                }

                if look_up_right(&input, x, y, width) {
                    sum += 1;
                }

                if look_down_left(&input, x, y, height) {
                    sum += 1;
                }

                if look_down_right(&input, x, y, width, height) {
                    sum += 1;
                }
            }
        }

        let mut second_sum = 0;

        for (y, line) in input.iter().enumerate() {
            for (x, letter) in line.iter().enumerate() {
                if *letter != 'A' {
                    continue;
                }

                if check_cross_mas(&input, x, y, width, height) {
                    second_sum += 1;
                }
            }
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
    fn challenge_four() {
        let challenge = ChallengeFour;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 18", "Second part: 9"]
        );
    }
}

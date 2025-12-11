use crate::{challenge::Challenge, Day};

pub struct ChallengeThree;

fn find_first_largest(text: &str) -> (usize, u8) {
    let mut largest = 0;
    let mut largest_index = 0;

    for (i, c) in text.chars().enumerate() {
        if c as u8 - b'0' > largest {
            largest = c as u8 - b'0';
            largest_index = i;
        }
    }

    (largest_index, largest)
}

fn find_n_largest_as_vec(text: &str, n: usize) -> Vec<u8> {
    if n == 0 {
        return vec![];
    }

    let chunk = &text[0..text.len() - n + 1];
    let (i, largest) = find_first_largest(chunk);

    let mut vec = vec![largest];
    vec.extend(find_n_largest_as_vec(&text[i + 1..], n - 1));
    vec
}

fn find_n_largest(text: &str, n: usize) -> u64 {
    find_n_largest_as_vec(text, n)
        .into_iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

impl Challenge for ChallengeThree {
    fn day(&self) -> crate::Day {
        Day::from(3).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let banks = input.trim().lines();
        let sum: u64 = banks.clone().map(|bank| find_n_largest(bank, 2)).sum();
        let second_sum: u64 = banks.map(|bank| find_n_largest(bank, 12)).sum();

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
    fn challenge_three() {
        let challenge = ChallengeThree;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 357", "Second part: 3121910778619"]
        );
    }
}

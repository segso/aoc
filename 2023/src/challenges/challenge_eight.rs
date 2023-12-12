use std::collections::HashMap;

use crate::{challenge::Challenge, Day};

pub fn lcm(numbers: &[u64]) -> u64 {
    let start = *numbers.iter().max().unwrap();
    let mut i = 0;

    'out: loop {
        i += start;
        for num in numbers {
            if i % num != 0 {
                continue 'out;
            }
        }

        break 'out;
    }

    i
}

pub struct ChallengeEight;

impl Challenge for ChallengeEight {
    fn day(&self) -> Day {
        Day::from(8).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let instructions = input.lines().next().unwrap().chars().collect::<Vec<_>>();

        let nodes_text = input.lines().skip(2).map(|line| {
            let mut buf = String::new();
            line.chars()
                .filter(|c| !c.is_ascii_punctuation())
                .for_each(|c| buf.push(c));

            let mut iter = buf.split_ascii_whitespace();
            (
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
            )
        });

        let mut nodes = HashMap::new();

        for (node, left, right) in nodes_text {
            nodes.insert(node, (left, right));
        }

        let mut current_node = "AAA";
        let mut position = 0;
        let mut steps = 0;

        while current_node != "ZZZ" {
            let instruction = instructions[position];

            let node = nodes.get(current_node).unwrap();
            if instruction == 'L' {
                current_node = &node.0;
            } else {
                current_node = &node.1;
            }

            steps += 1;
            position += 1;
            if position == instructions.len() {
                position = 0;
            }
        }

        let first_part_result = steps;

        let current_nodes = nodes
            .keys()
            .map(|node| node.to_string())
            .filter(|node| node.ends_with('A'))
            .collect::<Vec<_>>();
        let instructions_len = instructions.len();

        let steps = current_nodes
            .into_iter()
            .map(|mut node| {
                let mut steps = 0u64;
                let mut position = 0;

                while !node.ends_with('Z') {
                    let instruction = instructions[position];

                    if instruction == 'L' {
                        node = nodes[node.as_str()].0.clone();
                    } else {
                        node = nodes[node.as_str()].1.clone();
                    }

                    steps += 1;
                    position += 1;
                    if position == instructions_len {
                        position = 0;
                    }
                }

                steps
            })
            .collect::<Vec<_>>();

        let second_part_result = lcm(&steps);

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result:?}"),
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
            vec![format!("First part: 2"), format!("Second part: 6")]
        );
    }
}

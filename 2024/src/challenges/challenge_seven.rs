use crate::{challenge::Challenge, Day};

fn get_permutations(objects: &[&str], len: usize) -> Vec<Vec<String>> {
    let mut permutations = Vec::new();

    if len == 1 {
        for object in objects {
            permutations.push(vec![object.to_string()]);
        }

        return permutations;
    }

    let previous_permutations = get_permutations(objects, len - 1);

    for permutation in &previous_permutations {
        for operator in objects {
            let mut new_permutation = permutation.clone();
            new_permutation.push(operator.to_string());
            permutations.push(new_permutation);
        }
    }

    permutations
}

fn is_equation_possible(operators: &[&str], equation: &str) -> Option<u64> {
    let mut split = equation.split(':');
    let test_value = split.next().unwrap().parse::<u64>().unwrap();
    let operands = split
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|operand| operand.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let possible_operators = get_permutations(operators, operands.len() - 1);

    'out: for operators in possible_operators {
        let mut result = *operands.first().unwrap();

        for (i, operand) in operands.iter().copied().skip(1).enumerate() {
            match operators[i].as_str() {
                "+" => result += operand,
                "*" => result *= operand,
                "||" => result = format!("{result}{operand}").parse::<u64>().unwrap(),
                _ => {}
            }

            if result > test_value {
                continue 'out;
            }
        }

        if result == test_value {
            return Some(test_value);
        }
    }

    None
}

pub struct ChallengeSeven;

impl Challenge for ChallengeSeven {
    fn day(&self) -> crate::Day {
        Day::from(7).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let equations = input.lines().collect::<Vec<_>>();

        let mut sum = 0;

        for equation in &equations {
            if let Some(test_value) = is_equation_possible(&["+", "*"], equation) {
                sum += test_value;
            }
        }

        let mut second_sum = 0;

        for equation in &equations {
            if let Some(test_value) = is_equation_possible(&["+", "*", "||"], equation) {
                second_sum += test_value;
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
    fn challenge_seven() {
        let challenge = ChallengeSeven;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 3749", "Second part: 11387"]
        );
    }
}

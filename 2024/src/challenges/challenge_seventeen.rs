use crate::{challenge::Challenge, Day};

fn exec(
    registers: &mut (u64, u64, u64),
    output: &mut Vec<u64>,
    pointer: &mut usize,
    opcode: u64,
    literal: u64,
) {
    let combo = match literal {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!(),
    };

    match opcode {
        0 => registers.0 /= 2u64.pow(combo as u32),
        1 => registers.1 ^= literal,
        2 => registers.1 = combo % 8,
        3 => {
            if registers.0 != 0 {
                *pointer = literal as usize;
                *pointer = pointer.wrapping_sub(2);
            }
        }
        4 => registers.1 ^= registers.2,
        5 => output.push(combo % 8),
        6 => registers.1 = registers.0 / 2u64.pow(combo as u32),
        7 => registers.2 = registers.0 / 2u64.pow(combo as u32),
        _ => panic!(),
    }

    *pointer = pointer.wrapping_add(2)
}

pub struct ChallengeSeventeen;

impl Challenge for ChallengeSeventeen {
    fn day(&self) -> Day {
        Day::from(17).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let lines = input.lines().collect::<Vec<_>>();
        let mut registers = (0, 0, 0);

        registers.0 = lines[0].split(' ').last().unwrap().parse::<u64>().unwrap();
        registers.1 = lines[1].split(' ').last().unwrap().parse::<u64>().unwrap();
        registers.2 = lines[2].split(' ').last().unwrap().parse::<u64>().unwrap();

        let initial_registers = registers;

        let program = lines
            .last()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .replace(',', "")
            .chars()
            .map(|c| (c as u8 - b'0') as u64)
            .collect::<Vec<_>>();

        let mut pointer = 0;
        let mut output = Vec::new();

        while (0..program.len() - 1).contains(&pointer) {
            let opcode = program[pointer];
            let literal = program[pointer + 1];

            exec(&mut registers, &mut output, &mut pointer, opcode, literal);
        }

        let mut init = 0;
        let mut iterations = Vec::new();
        let mut differences = Vec::new();

        'outer: for i in 0.. {
            let mut pointer = 0;
            let mut output = Vec::new();

            registers = initial_registers;
            registers.0 = i;

            while (0..program.len() - 1).contains(&pointer) {
                let opcode = program[pointer];
                let literal = program[pointer + 1];

                exec(&mut registers, &mut output, &mut pointer, opcode, literal);

                if output.len() >= 7 && output[0..7] == program[0..7] && !iterations.contains(&i) {
                    if iterations.len() >= 2 {
                        let diff =
                            iterations[iterations.len() - 1] - iterations[iterations.len() - 2];
                        differences.push((i, diff));

                        let len = differences.len();

                        let diffs = differences.iter().map(|(_, diff)| diff).collect::<Vec<_>>();

                        if len % 2 == 0 && diffs[..len / 2] == diffs[len / 2..] {
                            break 'outer;
                        }
                    }
                    iterations.push(i);
                }

                if output.len() > program.len() {
                    continue 'outer;
                }

                if output.last().is_some() && *output.last().unwrap() != program[output.len() - 1] {
                    continue 'outer;
                }
            }

            if output == program {
                init = i;
                break;
            }
        }

        if !differences.is_empty() {
            let mut current = differences[0].0;
            let len = differences.len() / 2;
            let pattern = differences
                .into_iter()
                .take(len)
                .map(|(_, diff)| diff)
                .collect::<Vec<_>>();

            'outer: for i in 0.. {
                current += pattern[i % pattern.len()];
                let mut pointer = 0;
                let mut output = Vec::new();

                registers = initial_registers;
                registers.0 = current;

                while (0..program.len() - 1).contains(&pointer) {
                    let opcode = program[pointer];
                    let literal = program[pointer + 1];

                    exec(&mut registers, &mut output, &mut pointer, opcode, literal);

                    if output.len() > program.len() {
                        continue 'outer;
                    }

                    if output.last().is_some()
                        && *output.last().unwrap() != program[output.len() - 1]
                    {
                        continue 'outer;
                    }
                }

                if output == program {
                    init = current;
                    break;
                }
            }
        }

        vec![
            format!(
                "First part: {}",
                output
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!("Second part: {init}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_seventeen() {
        let challenge = ChallengeSeventeen;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 5,7,3,0", "Second part: 117440"]
        );
    }
}

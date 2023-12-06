use crate::{challenge::Challenge, Day};

pub struct ChallengeFive;

impl Challenge for ChallengeFive {
    fn day(&self) -> Day {
        Day::from(5).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let seeds = input
            .lines()
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|seed| seed.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut current_values = seeds.clone();

        let maps = input
            .split("\n\n")
            .skip(1)
            .map(|map| {
                map.lines()
                    .skip(1)
                    .map(|row| {
                        let [destination_start, source_start, length] = row
                            .split(' ')
                            .map(|num| num.parse::<u64>().unwrap())
                            .collect::<Vec<_>>()[..]
                        else {
                            unreachable!()
                        };
                        (destination_start, source_start, length)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        change_values(&maps, &mut current_values);
        let first_part_result = current_values.iter().min().unwrap();

        let mut min = u64::MAX;

        for i in (0..seeds.len()).step_by(2) {
            let start = seeds[i];
            let length = seeds[i + 1];

            for j in start..start + length {
                let mut values = vec![j];
                change_values(&maps, &mut values);
                if values[0] < min {
                    min = values[0];
                }
            }
        }

        let second_part_result = min;

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn change_values(maps: &[Vec<(u64, u64, u64)>], values: &mut [u64]) {
    for map in maps {
        values.iter_mut().for_each(|value| {
            for row in map {
                let (destination_start, source_start, length) = row;
                let source_end = source_start + length;

                if *value >= *source_start && *value < source_end {
                    if source_start < destination_start {
                        let difference = destination_start - source_start;
                        *value += difference;
                        return;
                    }
                    let difference = source_start - destination_start;
                    *value -= difference;
                    return;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_five() {
        let challenge = ChallengeFive;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec![format!("First part: 35"), format!("Second part: 46")]
        );
    }
}

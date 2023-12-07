use crate::{challenge::Challenge, Day};

pub struct ChallengeSix;

impl Challenge for ChallengeSix {
    fn day(&self) -> Day {
        Day::from(6).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut lines = input.lines();

        let values = lines
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .filter(|time| !time.is_empty())
            .map(|time| time.parse::<u64>().unwrap())
            .zip(
                lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .skip(1)
                    .filter(|distance| !distance.is_empty())
                    .map(|distance| distance.parse::<u64>().unwrap()),
            )
            .collect::<Vec<_>>();

        let first_part_result = values
            .iter()
            .map(|&(time, distance)| calculate_ways(time, distance))
            .product::<u64>();

        let values = values
            .iter()
            .map(|(time, distance)| (time.to_string(), distance.to_string()))
            .reduce(|(acc_time, acc_distance), (time, distance)| {
                (
                    format!("{acc_time}{time}"),
                    format!("{acc_distance}{distance}"),
                )
            })
            .unwrap();

        let second_part_result =
            calculate_ways(values.0.parse().unwrap(), values.1.parse().unwrap());

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
        ]
    }
}

fn calculate_ways(time: u64, distance: u64) -> u64 {
    let mut init = 0;
    for i in 0..=time {
        let milimeters = i * (time - i);
        if milimeters > distance {
            init = i;
            break;
        }
    }

    time - (init * 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_six() {
        let challenge = ChallengeSix;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec![format!("First part: 288"), format!("Second part: 71503")]
        );
    }
}

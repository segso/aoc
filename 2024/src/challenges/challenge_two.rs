use crate::{challenge::Challenge, Day};

enum Change {
    Undetermined,
    Increasing,
    Decreasing,
}

fn is_safe(report: &[u16]) -> bool {
    let mut levels = report.iter();
    let mut previous = levels.next().unwrap();
    let mut safe = true;

    let mut change = Change::Undetermined;

    for level in levels {
        let diff = previous.abs_diff(*level);
        if !(1..=3).contains(&diff) {
            safe = false;
            break;
        }

        match change {
            Change::Undetermined => {
                if previous > level {
                    change = Change::Decreasing;
                } else {
                    change = Change::Increasing;
                }
            }
            Change::Increasing => {
                if previous > level {
                    safe = false;
                    break;
                }
            }
            Change::Decreasing => {
                if previous < level {
                    safe = false;
                    break;
                }
            }
        }

        previous = level;
    }

    if safe {
        return true;
    }

    safe
}

pub struct ChallengeTwo;

impl Challenge for ChallengeTwo {
    fn day(&self) -> crate::Day {
        Day::from(2).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let reports = input.lines().map(|report| {
            report
                .split(' ')
                .map(|level| level.parse::<u16>().unwrap())
                .collect::<Vec<_>>()
        });

        let count = reports
            .clone()
            .map(|report| if is_safe(&report) { 1 } else { 0 })
            .sum::<u16>();

        let mut second_count = 0;

        for report in reports {
            if is_safe(&report) {
                second_count += 1;
                continue;
            }

            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);

                if is_safe(&new_report) {
                    second_count += 1;
                    break;
                }
            }
        }

        vec![
            format!("First part: {count}"),
            format!("Second part: {second_count}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_two() {
        let challenge = ChallengeTwo;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 2", "Second part: 4"]
        );
    }
}

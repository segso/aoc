use crate::{Day, challenge::Challenge};

pub struct ChallengeFive;

impl Challenge for ChallengeFive {
    fn day(&self) -> crate::Day {
        Day::from(5).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let (mut ranges, ids) = {
            let mut split = input.trim().split("\n\n");
            (
                split
                    .next()
                    .unwrap()
                    .lines()
                    .map(|range| {
                        let mut split = range.split('-');
                        let start = split.next().unwrap().parse::<u64>().unwrap();
                        let end = split.next().unwrap().parse::<u64>().unwrap();
                        start..=end
                    })
                    .collect::<Vec<_>>(),
                split.next().unwrap(),
            )
        };

        let count = ids
            .lines()
            .filter(|id| {
                let id = id.parse::<u64>().unwrap();
                ranges.iter().any(|range| range.contains(&id))
            })
            .count();

        let mut i = 0;
        'outer: while i < ranges.len() - 1 {
            let mut j = i + 1;
            while j < ranges.len() {
                if ranges[i].contains(ranges[j].start()) || ranges[j].contains(ranges[i].start()) {
                    let new_start = *ranges[i].start().min(ranges[j].start());
                    let new_end = *ranges[i].end().max(ranges[j].end());

                    ranges[i] = new_start..=new_end;
                    ranges.remove(j);
                    i = i.saturating_sub(1);
                    continue 'outer;
                }
                j += 1;
            }
            i += 1;
        }

        let sum: u64 = ranges
            .into_iter()
            .map(|range| range.end() - range.start() + 1)
            .sum();

        vec![
            format!("First part: {count}"),
            format!("Second part: {sum}"),
        ]
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
            vec!["First part: 3", "Second part: 14"]
        );
    }
}

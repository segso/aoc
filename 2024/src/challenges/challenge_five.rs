use crate::{challenge::Challenge, Day};

pub struct ChallengeFive;

impl Challenge for ChallengeFive {
    fn day(&self) -> crate::Day {
        Day::from(5).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut split = input.split("\n\n");
        let rules = split
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut split = line.split("|");
                (split.next().unwrap(), split.next().unwrap())
            })
            .collect::<Vec<_>>();

        let updates = split.next().unwrap();
        let mut incorrect_updates = Vec::new();

        let mut sum = 0;

        'out: for update in updates.lines() {
            let pages = update.split(',').collect::<Vec<_>>();
            let mut printed_pages = Vec::new();

            for page in &pages {
                printed_pages.push(page);

                let before_pages = rules
                    .iter()
                    .filter(|(_, after)| page == after)
                    .map(|(before, _)| before)
                    .filter(|before| update.split(',').any(|update| &&update == before))
                    .collect::<Vec<_>>();
                let after_pages = rules
                    .iter()
                    .filter(|(before, _)| page == before)
                    .map(|(_, after)| after)
                    .filter(|after| update.split(',').any(|update| &&update == after))
                    .collect::<Vec<_>>();

                if printed_pages[0..printed_pages.len() - 1]
                    .iter()
                    .any(|page| !before_pages.contains(page) || after_pages.contains(page))
                {
                    incorrect_updates.push(update);
                    continue 'out;
                }
            }

            sum += pages[pages.len() / 2].parse::<u32>().unwrap();
        }

        let mut second_sum = 0;

        for update in incorrect_updates {
            let mut pages = update.split(',').collect::<Vec<_>>();

            let mut curated_rules = rules
                .clone()
                .into_iter()
                .filter(|(before, after)| pages.contains(before) && pages.contains(after))
                .collect::<Vec<_>>();

            let mut new_update = Vec::new();

            while !pages.is_empty() {
                for i in 0..pages.len() {
                    let page = pages[i];

                    if curated_rules.iter().any(|(_, after)| &page == after) {
                        continue;
                    }

                    new_update.push(page);
                    pages.remove(i);

                    curated_rules = curated_rules
                        .into_iter()
                        .filter(|(before, after)| pages.contains(before) && pages.contains(after))
                        .collect::<Vec<_>>();

                    break;
                }
            }

            second_sum += new_update[new_update.len() / 2].parse::<u32>().unwrap();
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
    fn challenge_five() {
        let challenge = ChallengeFive;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 143", "Second part: 123"]
        );
    }
}

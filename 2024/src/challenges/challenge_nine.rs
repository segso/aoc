use crate::{challenge::Challenge, Day};

fn trim_end(vec: &mut Vec<Option<usize>>) -> bool {
    let mut has_changed = false;

    for i in (0..vec.len()).rev() {
        if vec[i].is_none() {
            vec.pop();
            has_changed = true;
        } else {
            break;
        }
    }

    has_changed
}

pub struct ChallengeNine;

impl Challenge for ChallengeNine {
    fn day(&self) -> crate::Day {
        Day::from(9).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let input = input.trim();
        let mut disk = Vec::new();

        let mut is_empty_space = false;

        for (id, digit) in input.chars().enumerate() {
            let digit = digit as u8 - b'0';

            let id = if is_empty_space { None } else { Some(id / 2) };

            for _ in 0..digit {
                disk.push(id);
            }

            is_empty_space = !is_empty_space;
        }

        loop {
            let Some(empty) = disk
                .iter()
                .enumerate()
                .find(|(_, block)| block.is_none())
                .map(|(id, _)| id)
            else {
                break;
            };

            let block = disk.pop().unwrap();

            disk[empty] = block;
            trim_end(&mut disk);
        }

        let sum = disk
            .iter()
            .enumerate()
            .map(|(id, block)| id * block.unwrap())
            .sum::<usize>();

        let mut disk = input
            .chars()
            .enumerate()
            .map(|(i, size)| {
                (
                    if i % 2 == 0 { Some(i / 2) } else { None },
                    (size as u8 - b'0') as usize,
                )
            })
            .collect::<Vec<_>>();

        let max_id = disk.last().unwrap().0.unwrap();

        for id in (0..=max_id).rev() {
            let (original_block_index, original_block) = disk
                .iter()
                .enumerate()
                .find(|(_, (i, _))| *i == Some(id))
                .unwrap();
            let id_size = original_block.1;

            let Some(index) = disk
                .iter()
                .enumerate()
                .find(|(_, (id, size))| id.is_none() && *size >= id_size)
            else {
                continue;
            };

            let hole_size = index.1 .1;
            let index = index.0;

            if index > original_block_index {
                continue;
            }

            disk[original_block_index] = (None, id_size);

            let mut new_disk = disk.iter().take(index).copied().collect::<Vec<_>>();
            new_disk.push((Some(id), id_size));

            if hole_size - id_size > 0 {
                new_disk.push((None, hole_size - id_size));
            }

            new_disk.extend(&disk[index + 1..]);
            disk = new_disk;
        }

        let mut expanded_disk = Vec::new();

        for block in disk {
            for _ in 0..block.1 {
                expanded_disk.push(block.0);
            }
        }

        let score = expanded_disk
            .iter()
            .enumerate()
            .filter(|(_, id)| id.is_some())
            .map(|(i, id)| i * id.unwrap())
            .sum::<usize>();

        vec![
            format!("First part: {sum}"),
            format!("Second part: {score}"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_nine() {
        let challenge = ChallengeNine;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 1928", "Second part: 2858"]
        );
    }
}

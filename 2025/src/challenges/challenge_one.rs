use crate::{challenge::Challenge, Day};

struct Dial {
    size: i32,
    count: i32,
    arrow: i32,
}

impl Dial {
    pub fn new(size: i32) -> Self {
        Dial {
            size,
            count: 0,
            arrow: 50,
        }
    }

    pub fn increment(&mut self) {
        self.arrow += 1;

        if self.arrow == self.size {
            self.arrow = 0;
            self.count += 1;
        }
    }

    pub fn decrement(&mut self) {
        self.arrow -= 1;

        if self.arrow == 0 {
            self.count += 1;
        }

        if self.arrow < 0 {
            self.arrow = self.size - 1;
        }
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

pub struct ChallengeOne;

impl Challenge for ChallengeOne {
    fn day(&self) -> crate::Day {
        Day::from(1).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        const DIAL_SIZE: i32 = 100;

        let deltas = input.lines().map(|line| {
            let num = line[1..].parse::<i32>().unwrap();

            (line.chars().next().unwrap(), num)
        });

        let mut arrow = 50;
        let mut first_sum = 0;

        for (direction, delta) in deltas.clone() {
            arrow += delta * if direction == 'L' { -1 } else { 1 };
            arrow %= DIAL_SIZE;

            if arrow.is_negative() {
                arrow += DIAL_SIZE;
            }

            if arrow == 0 {
                first_sum += 1;
            }
        }

        let mut dial = Dial::new(DIAL_SIZE);

        for (direction, delta) in deltas.clone() {
            let mut func: Box<dyn FnMut()> = if direction == 'L' {
                Box::new(|| dial.decrement())
            } else {
                Box::new(|| dial.increment())
            };

            for _ in 0..delta {
                func()
            }
        }

        vec![
            format!("First part: {first_sum}"),
            format!("Second part: {}", dial.count()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_one() {
        let challenge = ChallengeOne;
        let input = std::fs::read_to_string(challenge.path(crate::Environment::Local)).unwrap();

        assert_eq!(
            challenge.run(input),
            vec!["First part: 3", "Second part: 6"]
        );
    }
}

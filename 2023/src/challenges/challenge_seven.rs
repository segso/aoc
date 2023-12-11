use std::{cmp::Ordering, collections::HashMap};

use crate::{challenge::Challenge, Day};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Card {
    WeakJ,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::T => 'T',
            Self::J | Self::WeakJ => 'J',
            Self::Q => 'Q',
            Self::K => 'K',
            Self::A => 'A',
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    kind: HandKind,
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>, kind: HandKind) -> Self {
        Self { kind, cards }
    }

    fn kind(cards: &[Card]) -> HandKind {
        let mut count = HashMap::new();

        for card in cards {
            let c = card.to_char();
            let entry = count.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut counts = count.values().copied().collect::<Vec<_>>();
        counts.sort();

        match counts[..] {
            [5] => HandKind::FiveOfAKind,
            [1, 4] => HandKind::FourOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 1, 3] => HandKind::ThreeOfAKind,
            [1, 2, 2] => HandKind::TwoPair,
            [1, 1, 1, 2] => HandKind::OnePair,
            [1, 1, 1, 1, 1] => HandKind::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind != other.kind {
            return self.kind.cmp(&other.kind);
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ChallengeSeven;

impl Challenge for ChallengeSeven {
    fn day(&self) -> Day {
        Day::from(7).unwrap()
    }

    fn run(&self, input: String) -> Vec<String> {
        let mut hands = input
            .lines()
            .map(|line| {
                let cards = line.split_ascii_whitespace().next().unwrap();
                let bid = line
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let cards = cards.chars().map(Card::from).collect::<Vec<_>>();
                let kind = Hand::kind(&cards);
                let hand = Hand::new(cards, kind);
                (hand, bid)
            })
            .collect::<Vec<_>>();

        hands.sort();

        let first_part_result = (1..)
            .zip(hands.iter())
            .map(|(rank, (_, bid))| rank as u32 * bid)
            .sum::<u32>();

        let mut hands = input
            .lines()
            .map(|line| {
                let hand = line.split_ascii_whitespace().next().unwrap();
                let bid = line
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let mut max = HandKind::HighCard;

                let line_cards = hand.chars().map(Card::from).collect::<Vec<_>>();

                let cards = [
                    Card::A,
                    Card::K,
                    Card::Q,
                    Card::J,
                    Card::T,
                    Card::Nine,
                    Card::Eight,
                    Card::Seven,
                    Card::Six,
                    Card::Five,
                    Card::Four,
                    Card::Three,
                    Card::Two,
                ];

                for card in cards {
                    let line_cards = line_cards
                        .clone()
                        .into_iter()
                        .map(|line_card| {
                            if line_card == Card::J {
                                card.clone()
                            } else {
                                line_card
                            }
                        })
                        .collect::<Vec<_>>();

                    let kind = Hand::kind(&line_cards);

                    if max < kind {
                        max = kind;
                    }
                }

                let line_cards = line_cards
                    .into_iter()
                    .map(|card| if card == Card::J { Card::WeakJ } else { card })
                    .collect();

                (Hand::new(line_cards, max), bid)
            })
            .collect::<Vec<_>>();

        hands.sort();

        let second_part_result = (1..)
            .zip(hands.iter())
            .map(|(rank, (_, bid))| rank as u32 * bid)
            .sum::<u32>();

        vec![
            format!("First part: {first_part_result}"),
            format!("Second part: {second_part_result}"),
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
            vec![format!("First part: 6440"), format!("Second part: 5905")]
        );
    }
}

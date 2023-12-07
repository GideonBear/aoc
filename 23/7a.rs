//! ```cargo
//! [dependencies]
//! counter = "0.5.7"
//! itertools = "0.12.0"
//! ```

use std::fs;
use counter::Counter;
use itertools::Itertools;

const CARDS: &str = "23456789TJQKA";

#[derive(PartialEq, Eq, Debug)]
struct Hand(Vec<u8>);

impl Hand {
    fn rank(&self) -> (u8, &Vec<u8>) {
        let kinds = self.0
            .iter()
            .collect::<Counter<_>>()
            .k_most_common_ordered(2);
        (match kinds[..] {
            [(_, 5)] => 7,
            [(_, 4), ..] => 6,
            [(_, 3), (_, 2)] => 5,
            [(_, 3), ..] => 4,
            [(_, 2), (_, 2), ..] => 3,
            [(_, 2), ..] => 2,
            [(_, 1), ..] => 1,
            _ => panic!(),
        }, &self.0)
    }

    fn from_string(s: String) -> Self {
        Self(s
            .chars()
            .map(|c| CARDS.find(c).unwrap().try_into().unwrap())
            .collect()
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let text = fs::read_to_string("7.txt")
        .expect("Error while reading file");
    let text = text
        .strip_suffix('\n')
        .unwrap_or(&text);

    let mut hands: Vec<(Hand, usize)> = text.split('\n')
        .map(|line| line.split(' ').collect_tuple().unwrap())
        .map(|(hand, bid)| (
            Hand::from_string(hand.to_string()),
            bid.parse().unwrap(),
        ))
        .collect();
    hands.sort_unstable();
    let winnings: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (hand, bid))| bid * (i + 1))
        .sum();
    println!("{}", winnings)
}

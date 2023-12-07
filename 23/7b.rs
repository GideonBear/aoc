//! ```cargo
//! [dependencies]
//! counter = "0.5.7"
//! itertools = "0.12.0"
//! ```

use std::fs;
use counter::Counter;
use itertools::Itertools;
use std::collections::HashSet;

const CARDS: &str = "J23456789TQKA";

#[derive(PartialEq, Eq, Debug)]
struct Hand(Vec<u8>);

impl Hand {
    fn rank(&self) -> (u8, &Vec<u8>) {
        let kinds: HashSet<(&u8, usize)> = self.0
            .iter()
            .collect::<Counter<_>>()
            .into_iter()
            .collect();
        //println!("{kinds:?}");
        (
            if
                kinds.iter().any(|(_c, f)| f == &5)
                || kinds.iter().any(|(_c, f)| f == &4 && _c != &&0) && kinds.contains(&(&0, 1))
                || kinds.iter().any(|(_c, f)| f == &3 && _c != &&0) && kinds.contains(&(&0, 2))
                || kinds.iter().any(|(_c, f)| f == &2 && _c != &&0) && kinds.contains(&(&0, 3))
                || kinds.iter().any(|(_c, f)| f == &1 && _c != &&0) && kinds.contains(&(&0, 4))
            {7} else if
                kinds.iter().any(|(_c, f)| f == &4)
                || kinds.iter().any(|(_c, f)| f == &3 && _c != &&0) && kinds.contains(&(&0, 1))
                || kinds.iter().any(|(_c, f)| f == &2 && _c != &&0) && kinds.contains(&(&0, 2))
                || kinds.iter().any(|(_c, f)| f == &1 && _c != &&0) && kinds.contains(&(&0, 3))
            {6} else if
                kinds.iter().any(|(_c, f)| f == &3) && kinds.iter().any(|(_c, f)| f == &2)
                || {
                    let mut ret = false;
                    let mut the_c = None;
                    for (c, f) in &kinds {
                        if f == &2 {
                            the_c = Some(c);
                            break
                        }
                    }
                    if let Some(the_c) = the_c {
                        for (c, f) in &kinds {
                            if f == &2 && c != the_c {
                                ret = true;
                                break
                            }
                        }
                    }
                    ret
                }
                    && kinds.contains(&(&0, 1))
            {5} else if
                kinds.iter().any(|(_c, f)| f == &3)
                || kinds.iter().any(|(_c, f)| f == &2 && _c != &&0) && kinds.contains(&(&0, 1))
                || kinds.iter().any(|(_c, f)| f == &1 && _c != &&0) && kinds.contains(&(&0, 2))
            {4} else if
                    {
                        let mut ret = false;
                        let mut the_c = None;
                        for (c, f) in &kinds {
                            if f == &2 {
                                the_c = Some(c);
                                break
                            }
                        }
                        if let Some(the_c) = the_c {
                            for (c, f) in &kinds {
                                if f == &2 && c != the_c {
                                    ret = true;
                                    break
                                }
                            }
                        }
                        ret
                    }
            {3} else if
                kinds.iter().any(|(_c, f)| f == &2)
                || kinds.iter().any(|(_c, f)| f == &1 && _c != &&0) && kinds.contains(&(&0, 1))
            {2} else if
                kinds.iter().any(|(_c, f)| f == &1)
            {1} else {panic!()}
            , &self.0)
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
    //println!("{:?}", hands[0].0.rank());
    hands.sort_unstable();
    let winnings: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_hand, bid))| bid * (i + 1))
        .sum();
    println!("{winnings}")
}

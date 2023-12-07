//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! ```

use std::fs;
use itertools::Itertools;
use std::ops::Range;

struct Ma {
    from_cat: String,
    to_cat: String,
    ranges: Vec<(u16, u16, u16)>,
}

impl Ma {
    fn from_string(from_cat: String, to_cat: String, string: String) -> Self {
        return Self {
            from_cat,
            to_cat,
            ranges: string.split('\n')
                .map(|ma_range| {
                    ma_range
                        .split(' ')
                        .map(|x| x.parse().expect("Should be a number"))
                        .collect_tuple()
                        .expect("Number of items incorrect")
                })
                .collect(),
        }
    }

    fn get(&self, item: u16) -> u16 {
        for (dest_start, source_start, length) in self.ranges {
            if source_start <= item && item <= source_start + length {
                return dest_start + (item - source_start);
            }
        }
        item
    }
}

fn main() {
    let text = fs::read_to_string("5.txt").expect("Error while reading file");

    let groups = text.split("\n\n");
    let seeds: Vec<(u16, u16)> = groups
        .next().unwrap()
        .strip_prefix("seeds: ").unwrap()
        .split(' ')
        .map(|x| x.parse())
        .chunks(2)
        .into_iter()
        .map(|x| x.collect_tuple().unwrap())
        .collect();
    let total_seeds = seeds.iter().map(|(start, length)| length).sum();
    let seeds = seeds
        .into_iter()
        .map(|(start, length)| Range { start, end: start + length })
        .flatten();
    let old_mas = groups;
}

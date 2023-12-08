//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! indicatif = "0.17.7"
//! ```

use std::fs;
use itertools::Itertools;
use std::ops::Range;
use indicatif::ProgressBar;
use std::iter::Iterator;

struct ProgressWrapper<T, I: Iterator<Item = T>> {
    it: I,
    bar: ProgressBar,
}

impl<T, I: Iterator<Item = T>> ProgressWrapper<T, I> {
    fn new(it: I, total: u64) -> Self {
        Self {it, bar: ProgressBar::new(total)}
    }
}

impl<T, I: Iterator<Item = T>> Iterator for ProgressWrapper<T, I> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.bar.inc(1);
        self.it.next()
    }
}

struct Ma {
    from_cat: String,
    to_cat: String,
    ranges: Vec<(u32, u32, u32)>,
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
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .expect("Number of items incorrect")
                })
                .collect(),
        }
    }

    fn get(&self, item: u32) -> u32 {
        for (dest_start, source_start, length) in &self.ranges {
            if *source_start <= item && item <= source_start + length {
                return dest_start + (item - source_start);
            }
        }
        item
    }
}

fn main() {
    let text = fs::read_to_string("5e2.txt").expect("Error while reading file");

    let mut groups = text.split("\n\n");
    let seeds: Vec<(u32, u32)> = groups
        .next().unwrap()
        .strip_prefix("seeds: ").unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .chunks(2)
        .into_iter()
        .map(|x| x.collect_tuple().unwrap())
        .collect();
    let total_seeds = seeds
        .iter()
        .map(|(_start, length)| u64::from(*length))
        .sum();
    let mut seeds: Box<dyn Iterator<Item = u32>> = Box::new(seeds
        .into_iter()
        .map(|(start, length)| Range { start, end: start + length })
        .flatten());
    let old_mas = groups;

    let mut mas = vec![];
    for ma in old_mas {
        let dash_pos = ma.find("-").unwrap();
        let from_cat = &ma[..dash_pos];
        let ma = &ma[dash_pos + 4..];

        let space_pos = ma.find(" ").unwrap();
        let to_cat = &ma[..space_pos];
        let ma = &ma[space_pos + 6..];

        let ma = Ma::from_string(
            from_cat.to_string(),
            to_cat.to_string(),
            ma.to_string()
        );

        mas.push(ma);
    }

    for ma in mas {
        /*let s: Vec<_> = seeds.collect();
        println!("{s:?}");
        seeds = Box::new(s.into_iter());*/
        seeds = Box::new(seeds.map(move |x| ma.get(x)));
    }
    /*let s: Vec<_> = seeds.collect();
    println!("{s:?}");
    seeds = Box::new(s.into_iter());*/
    //if !proceed() {return}
    println!("{:?}", ProgressWrapper::new(seeds, total_seeds).min());
}

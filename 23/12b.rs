//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! ```

#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(iter_intersperse)]

use std::fs;
use itertools::Itertools;
use std::iter::repeat;

const FT: &[bool; 2] = &[false, true];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Spring {
    Fine,
    Damaged
}

impl Spring {
    fn from_bool(b: bool) -> Self {
        match b {
            true => Self::Damaged,
            false => Self::Fine,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum SpringInfo {
    Known(Spring),
    Unknown,
}

impl SpringInfo {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Known(Spring::Fine),
            '#' => Self::Known(Spring::Damaged),
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

fn matches_hints(row: impl Iterator<Item = Spring>, mut hints: impl Iterator<Item = &u32>) -> bool {
    let mut curr = 0;
    for item in row {
        match item {
            Spring::Fine => {
                if curr != 0 && hints.next() != Some(&curr) {
                    // println!("  - Doesn't match because the hint () wasn't the curr ({curr})");
                    return false
                }
                curr = 0;
            }
            Spring::Damaged => curr += 1,
        }
    }
    if curr != 0 && hints.next() != Some(&curr) {
        // println!("  - Doesn't match because after the last the hint () wasn't the curr ({curr})");
        return false
    }
    if let Some(x) = hints.next() {
        // println!("  - Doesn't match because hints weren't depleted (yielded {x})");
        return false
    }
    // println!("  - Matches");
    true
}

fn main() {
    let text = fs::read_to_string("12e2.txt").expect("Error while reading file");

    let rows = text
        .split('\n')
        .map(|line| line.split(' ').collect_tuple().unwrap())
        .map(|(row, hints)| {(
            row
                .chars()
                .map(SpringInfo::from_char)
                .collect(),
            hints
                .split(',')
                .map(|x| x.parse().unwrap()),
        )});
    let rows: Vec<(Vec<_>, Vec<_>)> = rows.map(|(row, hints)| {
        (
            Iterator::intersperse(repeat(row).take(5), vec![SpringInfo::Unknown]).flatten().collect(),
            repeat(hints).take(5).flatten().collect(),
        )
    }).collect();

    let result: u32 = rows
        .into_iter()
        .map(|(row, hints)| {
            let num_unknown = row.iter().filter(|&&x| x == SpringInfo::Unknown).count();
            println!("{num_unknown} unknown springs");
            let mut diff_arrs = 0;
            for t in repeat(FT).take(num_unknown).multi_cartesian_product() {
                // let mut stop = false;
                // println!("{t:?}");
                // println!("{row:?}");
                let mut t = t.into_iter();
                let new_row = row.iter().map(|&si| match si {
                        SpringInfo::Known(s) => {
                            // println!("Known({s:?}) found");
                            s
                        },
                        SpringInfo::Unknown => {
                            // println!("Unknown found");
                            Spring::from_bool(*t.next().unwrap())
                        },
                    }
                );
                // let new_row = new_row.collect::<Vec<_>>();
                // println!("- {:?}", new_row);
                // // if new_row == [Spring::Fine, Spring::Damaged, Spring::Fine, Spring::Damaged, Spring::Damaged, Spring::Damaged, Spring::Fine, Spring::Damaged, Spring::Fine, Spring::Damaged, Spring::Damaged, Spring::Damaged, Spring::Damaged, Spring::Damaged, Spring::Damaged] {
                // //     stop = true
                // // }
                // let new_row = new_row.into_iter();
                let matches = matches_hints(new_row, hints.iter());
                if matches {
                    assert_eq!(t.next(), None); // Only here because new_row is not fully consumed
                                                // by matches_hints without a match (it short-circuits)
                    diff_arrs += 1;
                }
                // if stop {
                //     std::process::exit(0);
                // }
            }
            println!("- {diff_arrs} different arrangements found");
            diff_arrs
        })
        .sum();

    println!("{result}");
}

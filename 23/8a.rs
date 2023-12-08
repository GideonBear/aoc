//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! ```

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let text = fs::read_to_string("8.txt").expect("Error while reading file");

    let mut lines = text.split('\n');
    //let directions = lines.next();
    let mut directions = lines
        .next().unwrap()
        .chars()
        .map(|d| match d {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .cycle();
    lines.next();
    let nodes: HashMap<&str, Vec<&str>> = lines
        .map(|s| s.split(" = ").collect_tuple().unwrap())
        .map(|(n, e)| (n, e
            .strip_prefix('(').unwrap()
            .strip_suffix(')').unwrap()
            .split(", ")
            .collect()))
        .collect();

    let mut curr = "AAA";
    let end = "ZZZ";
    let mut i = 0;
    while curr != end {
        i += 1;
        curr = nodes[curr][directions.next().unwrap()]
    }
    println!("{i}");
}

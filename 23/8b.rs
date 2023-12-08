//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
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

    //let mut curr_history: Vec<Vec<&&str>> = vec![];
    let mut curr: Vec<&&str> = nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .collect();
    let mut i = 0;

    while !curr.iter().all(|n| n.ends_with('Z')) {
        /*if curr_history.contains(&curr) {
            println!("{curr:?} is duped");
        }
        curr_history.push(curr.clone());*/
        i += 1;
        let curr_direction = directions.next().unwrap();
        curr = curr.into_iter().map(|n| &nodes[n][curr_direction]).collect();
    }
    println!("{i}");
}

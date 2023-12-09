//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! num = "0.4.1"
//! ```

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use num::integer::lcm;

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

    let starters: Vec<(&str, u32, u32)> = nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|&start| {
            // TODO: this is assuming that it only gets to a single endpoint, which is (probably) false
            let mut copied_directions = directions.clone();
            let mut curr = start;
            let mut dist_to_end: u32 = 0;
            while !curr.ends_with('Z') {
                dist_to_end += 1;
                curr = nodes[curr][copied_directions.next().unwrap()];
            }
            let endpoint = curr;
            println!("{endpoint}");
            let mut end_cycle_length: u32 = 0;
            end_cycle_length += 1;
            curr = nodes[curr][copied_directions.next().unwrap()];
            while curr != endpoint {
                if curr.ends_with('Z') {
                    println!("!! Second endpoint {curr} found when expecting endpoint {endpoint} !!");
                }
                end_cycle_length += 1;
                curr = nodes[curr][copied_directions.next().unwrap()];
            }

            (start, dist_to_end, end_cycle_length)
        })
        .collect();
    println!("{starters:?}");

    let starter_with_largest_dist_to_end = starters.iter().max_by_key(|x| x.1).unwrap();
    let largest_dist_to_end = starter_with_largest_dist_to_end.1;
    let states = starters.iter().map(|(start, dist_to_end, end_cycle_length)| {
        let passed = largest_dist_to_end - dist_to_end;
        let passed_in_cycle = passed % end_cycle_length;
        (start, end_cycle_length, passed_in_cycle)
    }).collect::<Vec<_>>();
    println!("{:?}", states);

    let mut i = 0;
    loop {
        i += 1;
        if states.iter().all(|(&start, &end_cycle_length, passed_in_cycle)| i % end_cycle_length == *passed_in_cycle) {
            println!("{i} is good, meaning {} is the answer", i + largest_dist_to_end);
            break
        }
    }

    //let mut curr_history: Vec<Vec<&&str>> = vec![];
    /*let mut i = 0;

    while !curr.iter().all(|n| n.ends_with('Z')) {
        /*if curr_history.contains(&curr) {
            println!("{curr:?} is duped");
        }
        curr_history.push(curr.clone());*/
        i += 1;
        let curr_direction = directions.next().unwrap();
        curr = curr.into_iter().map(|n| &nodes[n][curr_direction]).collect();
    }
    println!("{i}");*/
}

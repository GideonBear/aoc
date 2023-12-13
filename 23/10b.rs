//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! ```

#![feature(iter_intersperse)]

use std::fs;
use grid::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum PipeVector {
    Start,
    Ground,
    Vectors((isize, isize), (isize, isize)),
}

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const EAST: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (0, -1);

impl PipeVector {
    fn new(s: char) -> Self {
        match s {
            '|' => Self::Vectors(NORTH, SOUTH),
            '-' => Self::Vectors(EAST, WEST),
            'L' => Self::Vectors(NORTH, EAST),
            'J' => Self::Vectors(NORTH, WEST),
            '7' => Self::Vectors(SOUTH, WEST),
            'F' => Self::Vectors(SOUTH, EAST),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!(),
        }
    }
}

fn grid_vec<T>(grid: &Grid<T>, coords: &(usize, usize), vec: &(isize, isize)) -> Option<(usize, usize)> {
    let &(row, col) = coords;
    let new_row = isize::try_from(row).unwrap() + vec.0;
    let new_col = isize::try_from(col).unwrap() + vec.1;
    let (grid_height, grid_width) = grid.size();
    if new_row < 0 || new_col < 0 ||
        new_row > grid_height.try_into().unwrap() || new_col > grid_width.try_into().unwrap() {
        return None;
    }
    Some((new_row.try_into().unwrap(), new_col.try_into().unwrap()))
}

fn neg_vec(vec: &(isize, isize)) -> (isize, isize) {
    (-vec.0, -vec.1)
}

fn num_enclosed(grid: &Grid<PipeVector>, in_loop: Vec<(usize, usize)>) -> usize {
    grid
        .indexed_iter()
        .map(|(coords, _item)| coords)
        .filter(|&coords| {
            if in_loop.contains(&coords) { false }
            else {
                let (row, col) = coords;
                let mut inside = false;
                let mut last_lr_vec = None;
                println!("Trying {row}, {col}");
                for i in 0..row {
                    if !in_loop.contains(&(i, col)) {
                        continue
                    }
                    println!("It {i}, {col}");
                    if let PipeVector::Start = grid[(i, col)] {
                        panic!();
                    }
                    if let PipeVector::Vectors(mut vec1, mut vec2) = grid[(i, col)] {
                        if (vec1 == EAST || vec1 == WEST) && vec2 != WEST {
                            println!("FLIPPING (ONLY ON START!!)");
                            (vec1, vec2) = (vec2, vec1);
                        }
                        println!("{vec1:?}, {vec2:?}");
                        if vec1 == EAST && vec2 == WEST {
                            println!("Found east-west pipe, flipping inside");
                            inside = !inside;
                        } else if vec2 == EAST || vec2 == WEST {
                            match last_lr_vec{
                                Some(x) if x == neg_vec(&vec2) => {
                                    println!("Found neg_vec(vec2) == last_lr_vec, flipping inside");
                                    inside = !inside;
                                    last_lr_vec = None;
                                }
                                Some(x) => {
                                    println!("Resetting last_lr_vec");
                                    assert_eq!(vec2, x, "vec2 should be the same as last_lr_vec");
                                    last_lr_vec = None;
                                }
                                None => {
                                    println!("Setting last_lr_vec");
                                    last_lr_vec = Some(vec2);
                                }
                            }
                        } else {
                            println!("North-South pipe found");
                        }
                    } else {
                        println!("{:?} is not a Vectors", grid[(i, col)]);
                    }
                }
                println!("inside={inside}");
                inside
            }
        })
        .count()
}

fn main() {
    let text = fs::read_to_string("10.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(PipeVector::new).collect(), width
    );

    let (start, _) = grid.indexed_iter().find(|((_row, _col), vec)| vec == &&PipeVector::Start).unwrap();
    println!("start at {start:?}");

    let mut in_loop: HashMap<(usize, usize), u32> = HashMap::new();
    let mut curr = start;
    let mut last_vec = (0, 0);
    let mut starting_vecs_done = vec![];
    let mut dist = 0;
    let mut start_connections = vec![];
    loop {
        in_loop.entry(curr).and_modify(|old_dist| *old_dist = dist.min(*old_dist)).or_insert(dist);
        if in_loop.contains_key(&curr) {
            //println!("Replacing {} with {}, dist={dist}", in_loop[&curr], dist.min(in_loop[&curr]));
            in_loop.insert(curr, dist.min(in_loop[&curr]));
        } else {
            in_loop.insert(curr, dist);
        }
        //dbg!(&in_loop);
        match grid[curr] {
            PipeVector::Start => {
                let mut done = true;
                dist = 0;
                for vec in [NORTH, EAST, SOUTH, WEST] {
                    if starting_vecs_done.contains(&vec) {
                        //println!("Skipping {vec:?}");
                        continue;
                    }
                    match grid_vec(&grid, &curr, &vec) {
                        Some(coords) => {
                            println!("Found possible starting point at {coords:?}");
                            starting_vecs_done.push(vec);
                            match grid[coords] {
                                PipeVector::Vectors(vec1, vec2) => {
                                    if vec1 == neg_vec(&vec) || vec2 == neg_vec(&vec) {
                                        println!("Was correct starting point, storing curr, last_vec, incrementing dist, and exiting");
                                        curr = coords;
                                        last_vec = vec;
                                        dist += 1;
                                        done = false;
                                        start_connections.push(
                                            if vec1 == neg_vec(&vec) { neg_vec(&vec1) } else { neg_vec(&vec2) }
                                        );
                                        break;
                                    }
                                    println!("Neither vec ({vec1:?}, {vec2:?}) matching neg vec ({vec:?}), trying next");
                                }
                                PipeVector::Ground => (),
                                PipeVector::Start => panic!(),
                            }
                        }
                        None => (),
                    }
                }
                if done {
                    println!("Done; breaking");
                    break;
                }
            }
            PipeVector::Vectors(vec1, vec2) => {
                let vec;
                if vec1 == neg_vec(&last_vec) {
                    vec = vec2;
                } else if vec2 == neg_vec(&last_vec) {
                    vec = vec1;
                } else {
                    panic!("Neither vec ({vec1:?}, {vec2:?}) matching neg last_vec ({last_vec:?})");
                }
                //println!("vec = {vec:?}");
                match grid_vec(&grid, &curr, &vec) {
                    None => panic!("Vector {vec:?} invalid"),
                    Some(coords) => {
                        println!("Found next at {coords:?}, storing curr, last_vec, incrementing dist");
                        curr = coords;
                        last_vec = vec;
                        dist += 1;
                    },
                }
            }
            PipeVector::Ground => unreachable!(),
        }
    }

    println!("start_connections={start_connections:?}");
    let mut done = false;
    for coords in in_loop.keys() {
        if let PipeVector::Start = grid[*coords] {
            grid[*coords] = PipeVector::Vectors(start_connections[0], start_connections[1]);
            done = true;
            break;
        }
    }
    assert!(done);

    println!("{in_loop:?}");
    let answer = num_enclosed(&grid, in_loop.into_keys().collect());
    println!("{answer:?}");
}

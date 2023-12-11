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

fn num_enclosed(grid: &Grid<PipeVector>, in_loop: impl Iterator<Item = (usize, usize)>) -> usize {
    grid
        .indexed_iter()
        .map(|(coords, _item)| coords)
        .filter(|&coords| {
            if grid[coords] != PipeVector::Ground { false }
            else {
                // TODO: connected pipes should be skipped somehow? See 8, 9 in eb1
                let (row, col) = coords;
                let mut inside = false;
                println!("Trying {row}, {col}");
                for i in 0..row {
                    println!("It {i}, {col}");
                    if let PipeVector::Vectors(_, _) = grid[(i, col)] {
                        println!("Found pipe, flipping inside");
                        inside = !inside;
                    }
                }
                println!("inside={inside}");
                inside
            }
        })
        .map(|_| 1)
        .sum()
}

fn main() {
    let text = fs::read_to_string("10eb1.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let grid = Grid::from_vec(
        lines.join("").chars().map(PipeVector::new).collect(), width
    );

    let (start, _) = grid.indexed_iter().find(|((_row, _col), vec)| vec == &&PipeVector::Start).unwrap();
    println!("start at {start:?}");

    let mut in_loop: HashMap<(usize, usize), u32> = HashMap::new();
    let mut curr = start;
    let mut last_vec = (0, 0);
    let mut starting_vecs_done = vec![];
    let mut dist = 0;
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

    println!("{in_loop:?}");
    let answer = num_enclosed(&grid, in_loop.into_keys());
    println!("{answer:?}");
}

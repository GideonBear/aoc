//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! ```

#![feature(iter_intersperse)]

use std::fs;
use grid::*;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
enum PipeVector {
    Start,
    Ground,
    Vectors((isize, isize), (isize, isize)),
}

const NORTH: (isize, isize) = (1, 0);
const SOUTH: (isize, isize) = (-1, 0);
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

fn grid_vec(grid: &Grid<usize>, row: usize, col: usize, vec: &(isize, isize)) -> Option<(usize, usize)> {
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

fn main() {
    let text = fs::read_to_string("10e1.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(PipeVector::new).collect(), width
    );

    let (start, _) = grid.indexed_iter().find(|((_row, _col), vec)| vec == &&PipeVector::Start).unwrap();
    println!("start at {start:?}");

    let mut in_loop = vec![];
    let curr = start;
    let last_vec = (0, 0);
    while !in_loop.contains(&curr) {
        in_loop.push(curr);
        match grid[curr] {
            PipeVector::Start => {
                for vec in [NORTH, EAST, SOUTH, WEST] {
                    todo!();
                }
            }
            PipeVector::Vectors(vec1, vec2) => {
                let vec;
                if vec1 == neg_vec(&last_vec) {
                    vec = vec2;
                } else if vec2 == neg_vec(&last_vec) {
                    vec = vec1;
                } else {
                    panic!();
                }
            }
            PipeVector::Ground => unreachable!(),
        }
    }

    todo!();
}

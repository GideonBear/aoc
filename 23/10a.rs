//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! ```

#![feature(iter_intersperse)]

use std::fs;
use grid::*;
use itertools::Itertools;

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

fn grid_vec(grid: &Grid, vec: &(isize, isize)) -> Option<(usize, usize)> {
    todo!();
}

fn main() {
    let text = fs::read_to_string("10e1.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(PipeVector::new).collect(), width
    );

    todo!();
}

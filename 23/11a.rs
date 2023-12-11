//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! ```

use std::fs;
use grid::*;
use itertools::Itertools;
use std::iter::repeat;

/*
const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const EAST: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (0, -1);
const DIRS: [(isize, isize); 4] = [NORTH, SOUTH, EAST, WEST];
 */

fn parse_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!(),
    }
}

fn expand(grid: &mut Grid<bool>) {
    let mut i = 0;
    while i < grid.rows() {
        if grid.iter_row(i).all(|x| !x) {
            println!("Inserting row at {i}");
            grid.insert_row(i, repeat(false).take(grid.cols()).collect());
            i += 1;
        }
        i += 1;
    }
    let mut i = 0;
    while i < grid.cols() {
        if grid.iter_col(i).all(|x| !x) {
            println!("Inserting col at {i}");
            grid.insert_col(i, repeat(false).take(grid.rows()).collect());
            i += 1;
        }
        i += 1;
    }
}

fn main() {
    let text = fs::read_to_string("11.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(parse_char).collect(), width
    );
    expand(&mut grid);
    let galaxies: Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|(coords, &x)| x)
        .map(|(coords, x)| coords)
        .collect();

    let answer: usize = galaxies
        .iter()
        .combinations(2)
        .map(|x| x.into_iter().collect_tuple().unwrap())
        .map(|((r1, c1), (r2, c2))| {
            let r1 = isize::try_from(*r1).unwrap();
            let c1 = isize::try_from(*c1).unwrap();
            let r2 = isize::try_from(*r2).unwrap();
            let c2 = isize::try_from(*c2).unwrap();
            (r1 - r2).unsigned_abs() + (c1 - c2).unsigned_abs()
        })
        .sum();

    println!("{answer}");
}

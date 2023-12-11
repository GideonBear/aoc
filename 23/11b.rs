//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use grid::*;
use itertools::Itertools;
use tqdm::tqdm;

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

fn get_expanded(grid: &Grid<bool>) -> (Vec<usize>, Vec<usize>) {
    (
        grid.iter_rows().enumerate().filter(|(_i, row)| row.clone().all(|x| !x)).map(|(i, _row)| i).collect(),
        grid.iter_cols().enumerate().filter(|(_i, row)| row.clone().all(|x| !x)).map(|(i, _row)| i).collect(),
    )
}

fn main() {
    let text = fs::read_to_string("11.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(parse_char).collect(), width
    );
    let (expanded_rows, expanded_cols) = get_expanded(&mut grid);
    // println!("{expanded_rows:?}");
    // println!("{expanded_cols:?}");
    let galaxies: Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|(coords, &x)| x)
        .map(|(coords, x)| coords)
        .collect();

    let answer: usize = galaxies
        .iter()
        .combinations(2)
        .map(|x| x.into_iter().collect_tuple().unwrap())
        .map(|(&(r1, c1), &(r2, c2))| {
            // Should be x1 - 1..x2 (exclusive) or x1..=x2 (inclusive),
            // but doesn't matter since the row/col the galaxy is on can never be
            let row_added: usize = (r1.min(r2)..r1.max(r2)).into_iter().filter(|x| expanded_rows.contains(x)).map(|_| 999_999).sum();
            let col_added: usize = (c1.min(c2)..c1.max(c2)).into_iter().filter(|x| expanded_cols.contains(x)).map(|_| 999_999).sum();
            let r1 = isize::try_from(r1).unwrap();
            let c1 = isize::try_from(c1).unwrap();
            let r2 = isize::try_from(r2).unwrap();
            let c2 = isize::try_from(c2).unwrap();
            println!(
                "{:?}: +{}, +{}: {}",
                ((r1, c1), (r2, c2)),
                row_added,
                col_added,
                (r1 - r2).unsigned_abs() + row_added + (c1 - c2).unsigned_abs() + col_added);
            (r1 - r2).unsigned_abs() + row_added + (c1 - c2).unsigned_abs() + col_added
        })
        .sum();

    println!("{answer}");
}

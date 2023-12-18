//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use grid::*;
use itertools::Itertools;

type Coords = (usize, usize);
type Vector = (isize, isize);

const NORTH: Vector = (-1, 0);
const SOUTH: Vector = (1, 0);
const EAST: Vector = (0, 1);
const WEST: Vector = (0, -1);
const DIRS: [Vector; 4] = [NORTH, SOUTH, EAST, WEST];

fn add_vec<T>(grid: &Grid<T>, (c1, c2): Coords, (v1, v2): Vector) -> Option<Coords> {
    let n1 = isize::try_from(c1).unwrap() + v1;
    let n2 = isize::try_from(c2).unwrap() + v2;
    if n1 < 0
        || n2 < 0
        // || n1 >= grid.rows().try_into().unwrap() // These should be reported as errors so I can resize
        // || n2 >= grid.cols().try_into().unwrap()
    {
        None
    } else {
        Some((
            usize::try_from(n1).unwrap(),
            usize::try_from(n2).unwrap(),
        ))
    }
}

fn mult_vec((v1, v2): Vector, x: isize) -> Vector {
    (v1 * x, v2 * x)
}

fn parse_dir(s: &str) -> Vector {
    match s {
        "U" => NORTH,
        "D" => SOUTH,
        "R" => EAST,
        "L" => WEST,
        _ => panic!(),
    }
}

fn num_enclosed(grid: &Grid<bool>) -> usize {
    grid
        .indexed_iter()
        .filter(|&(coords, &item)| {
            println!("{coords:?}");
            if item { false }
            else {
                let (row, col) = coords;
                let mut inside = false;
                let mut last_was = false;
                let mut connection_was_right = false;
                for i in 0..row {
                    if grid[(i, col)] {
                        if !last_was {
                            inside = !inside;
                            last_was = true;
                            connection_was_right = grid[(i, col + 1)] == true;
                        } else {
                            if grid[(i + 1, col)] == false
                                && grid[(i, col + 1)] == connection_was_right
                            {
                                inside = !inside;
                            }
                        }
                    } else {
                        last_was = false;
                    }
                }
                println!("{inside}");
                inside
            }
        })
        .count()
}

fn main() {
    let text = fs::read_to_string("18.txt").expect("Error while reading file");

    let steps = text
        .split('\n')
        .map(|line| line.split(' ').collect_tuple().unwrap())
        .map(|(dir, len, color)| {
            (
                parse_dir(dir),
                len.parse::<usize>().unwrap(),
                color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap(),
            )
        });

    let mut grid: Grid<bool> = Grid::new(1000, 1000); // Change size when necessary
    let start = (grid.rows() / 2, grid.cols() / 2); // Trash
    grid[start] = true;

    let mut curr = start;
    for (i, (dir, len, color)) in steps.enumerate() {
        println!("Step {i}");
        for i in 0..len {
            curr = add_vec(&grid, curr, dir).expect("Instructions incorrect");
            grid[curr] = true;
        }
    }
    let num_trench = grid.iter().filter(|&&x| x).count();
    let num_enclosed = num_enclosed(&grid);
    println!("{} + {} = {}", num_trench, num_enclosed, num_trench + num_enclosed);
}

//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use grid::*;
use std::collections::{HashSet, VecDeque};
// use itertools::Itertools;
// use tqdm::tqdm;

type Coords = (usize, usize);
type Vector = (isize, isize);

const NORTH: Vector = (-1, 0);
const SOUTH: Vector = (1, 0);
const EAST: Vector = (0, 1);
const WEST: Vector = (0, -1);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Space {
    Empty,
    MirrorForward,
    MirrorBack,
    SplitterVertical,
    SplitterHorizontal,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::MirrorForward,
            '\\' => Self::MirrorBack,
            '|' => Self::SplitterVertical,
            '-' => Self::SplitterHorizontal,
            _ => panic!(),
        }
    }

    fn move_through(&self, dir: Vector) -> Vec<Vector> {
        // println!("  - Found {self:?}");
        match (self, dir) {
            (Self::Empty, dir) => vec![dir],

            (Self::MirrorForward, NORTH) => vec![EAST],
            (Self::MirrorForward, EAST) => vec![NORTH],
            (Self::MirrorForward, SOUTH) => vec![WEST],
            (Self::MirrorForward, WEST) => vec![SOUTH],

            (Self::MirrorBack, NORTH) => vec![WEST],
            (Self::MirrorBack, EAST) => vec![SOUTH],
            (Self::MirrorBack, SOUTH) => vec![EAST],
            (Self::MirrorBack, WEST) => vec![NORTH],

            (Self::SplitterVertical, NORTH | SOUTH) => vec![dir],
            (Self::SplitterHorizontal, EAST | WEST) => vec![dir],

            (Self::SplitterVertical, EAST | WEST) => vec![NORTH, SOUTH],
            (Self::SplitterHorizontal, NORTH | SOUTH) => vec![EAST, WEST],

            _ => panic!(),
        }
    }
}

fn add_vec<T>(grid: &Grid<T>, (c1, c2): Coords, (v1, v2): Vector) -> Option<Coords> {
    let n1 = isize::try_from(c1).unwrap() + v1;
    let n2 = isize::try_from(c2).unwrap() + v2;
    if n1 < 0
        || n2 < 0
        || n1 >= grid.rows().try_into().unwrap()
        || n2 >= grid.cols().try_into().unwrap()
    {
        None
    } else {
        Some((
            usize::try_from(n1).unwrap(),
            usize::try_from(n2).unwrap(),
        ))
    }
}

fn try_configuration(grid: &Grid<Space>, sc: Coords, sd: Vector) -> usize {
    println!("Trying {sc:?}, {sd:?}");
    let mut todo = VecDeque::from([(sc, sd)]);
    let mut history = vec![];
    let mut energized = HashSet::new();

    let (loc, dir) = todo.pop_front().unwrap();
    energized.insert(loc);
    let space = grid[loc];
    todo.extend(space
        .move_through(dir)
        .into_iter()
        .map(|vec| (loc, vec))
    );

    while !todo.is_empty() {
        let (loc, dir) = todo.pop_front().unwrap();
        if history.contains(&(loc, dir)) {
            // println!("- In history, skipping");
            continue;
        }
        history.push((loc, dir));
        match add_vec(&grid, loc, dir) {
            None => {
                // println!("- Beam direction off-grid, skipping")
            }
            Some(nloc) => {
                // println!("- Beam passed through {nloc:?}");
                // println!("  - Which should contain {:?}", grid[nloc]);
                energized.insert(nloc);
                let space = grid[nloc];
                todo.extend(space
                    .move_through(dir)
                    .into_iter()
                    .map(|vec| (nloc, vec))
                );
            }
        }
        // for (i, row) in grid.iter_rows().map(|x| x.enumerate()).enumerate() {
        //     println!("{}", row.map(|(j, _)| if energized.contains(&(i, j)) { '#' } else { '.' }).collect::<String>());
        // }
        // println!("  - Currently energized: {}", energized.len());
    }
    println!("- Total energized this run: {}", energized.len());
    energized.len()
}

fn main() {
    let text = fs::read_to_string("16.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.collect::<String>().chars().map(Space::from_char).collect(), width
    );

    let result = (0..grid.rows()).map(|x| ((x, 0), EAST))
        .chain((0..grid.rows()).map(|x| ((x, grid.cols() - 1), WEST)))
        .chain((0..grid.cols()).map(|x| ((0, x), SOUTH)))
        .chain((0..grid.cols()).map(|x| ((grid.rows() - 1, x), NORTH)))
        .map(|(sc, sd)| try_configuration(&grid, sc, sd))
        .max();

    println!("{result:?}");
}

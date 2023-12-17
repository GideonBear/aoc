//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use grid::*;
// use std::collections::{HashSet, VecDeque};
// use itertools::Itertools;
// use tqdm::tqdm;

type Coords = (usize, usize);
type Vector = (isize, isize);

const NORTH: Vector = (-1, 0);
const SOUTH: Vector = (1, 0);
const EAST: Vector = (0, 1);
const WEST: Vector = (0, -1);

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

struct Graph {
    nodes: Vec<Node>,
    start: Box<Node>,
}

impl Graph {
    fn from_grid(grid: &Grid<u8>, start_coords: Coords) -> Self {
        let mut nodes = vec![];
        let mut start = Node::from_grid(&mut nodes, &grid, start_coords);
        Self {
            nodes,
            start: Box::new(start),
        }
    }
}

struct Node {
    coords: Coords,
    edges: Vec<(u8, Vector, Box<Node>)>,
}

impl Node {
    fn new(coords: Coords) -> Self {
        Self {
            coords,
            edges: vec![],
        }
    }

    fn from_grid(nodes: &mut Vec<Node>, grid: &Grid<u8>, coords: Coords) -> Self {
        todo!();
    }
}

fn main() {
    let text = fs::read_to_string("17.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.collect::<String>().chars().map(|x| x.to_digit(10).unwrap().try_into().unwrap()).collect(), width
    );

    let start = (0, 0);
    let end = (grid.rows() - 1, grid.cols() - 1);
    let max_straight_line = 3;
    let graph = Graph::from_grid(&grid, start);
    // TODO: somehow do the three straight moves rule
}

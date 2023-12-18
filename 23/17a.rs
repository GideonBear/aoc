//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

#![feature(iter_collect_into)]

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
const DIRS: [Vector; 4] = [NORTH, SOUTH, EAST, WEST];

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

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    start: usize,
}

impl Graph {
    fn get_node(nodes: &Vec<Coords>, coords: Coords) -> usize {
        nodes
            .iter()
            .position(|&x| x == coords)
            .expect("Node at {coords:?} doesn't exist in {nodes:?}")
    }

    fn from_grid(grid: &Grid<u8>, start_coords: Coords) -> Self {
        let mut nodes = vec![];

        // Add all nodes with empty edges
        for (coords, val) in grid.indexed_iter() {
            nodes.push(Node {
                coords,
                edges: vec![]
            })
        }
        let nodes_coords = nodes.iter().map(|x| x.coords).collect::<Vec<_>>();

        // Populate edges
        for node in nodes.iter_mut() {
            DIRS
                .iter()
                .map(|&dir| match add_vec(grid, node.coords, dir) {
                    Some(new_coords) => {
                        println!("Found new node via dir {dir:?} at {:?}: {new_coords:?}", node.coords);
                        let new_val = grid[new_coords];
                        Some((
                            new_val,
                            dir,
                            Self::get_node(&nodes_coords, new_coords),
                        ))
                    }
                    None => None,
                })
                // .filter(|x| x.is_some()).map(|x| x.unwrap())
                .flatten()
                .collect_into(&mut node.edges);
        }

        let start = Self::get_node(&nodes_coords, start_coords);

        Self {
            nodes,
            start,
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    coords: Coords,
    edges: Vec<(u8, Vector, usize)>,
}

fn main() {
    let text = fs::read_to_string("17e.txt").expect("Error while reading file");

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

//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use grid::*;
use itertools::Itertools;
use std::collections::VecDeque;
use tqdm::tqdm;

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const EAST: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (0, -1);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Round,
    Square,
    Empty,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Square,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

fn tilt(grid: &mut Grid<Space>, dir: (isize, isize)) {
    let do_after: Box<dyn Fn (&mut Grid<Space>) -> ()> = match dir {
        (-1, 0) => { Box::new(|_grid| ()) }
        (1, 0) => { grid.rotate_half(); Box::new(|grid| grid.rotate_half()) }
        (0, 1) => { grid.rotate_left(); Box::new(|grid| grid.rotate_right()) }
        (0, -1) => { grid.rotate_right(); Box::new(|grid| grid.rotate_left()) }
        _ => panic!(),
    };
    let mut todo: VecDeque<_> = grid
        .indexed_iter()
        .filter(|((row, _col), &space)| *row != 0 && space == Space::Round)
        .map(|(coords, _space)| coords)
        .collect();
    while !todo.is_empty() {
        let (row, col) = todo.pop_front().unwrap();
        // println!("Doing {row}, {col}");
        if grid[(row - 1, col)] == Space::Empty {
            // println!("- Should fall, falling");
            grid[(row - 1, col)] = Space::Round;
            grid[(row, col)] = Space::Empty;
            if row - 1 == 0 { continue }
            todo.push_back((row - 1, col));
        }
    }
    do_after(grid);
}

fn spin_cycle(grid: &mut Grid<Space>) {
    tilt(grid, NORTH);
    tilt(grid, WEST);
    tilt(grid, SOUTH);
    tilt(grid, EAST);
}

fn get_load(grid: &mut Grid<Space>, dir: (isize, isize)) -> usize {
    //! I promise not to modify `grid`
    let do_after: Box<dyn Fn (&mut Grid<Space>) -> ()> = match dir {
        (-1, 0) => { Box::new(|_grid| ()) }
        (1, 0) => { grid.rotate_half(); Box::new(|grid| grid.rotate_half()) }
        (0, 1) => { grid.rotate_left(); Box::new(|grid| grid.rotate_right()) }
        (0, -1) => { grid.rotate_right(); Box::new(|grid| grid.rotate_left()) }
        _ => panic!(),
    };
    let load = grid
        .indexed_iter()
        .filter(|(_coords, &space)| space == Space::Round)
        .map(|((row, col), space)| { /*println!("{row}, {col}, {space:?}");*/ ((row, col), space) })
        .map(|((row, _col), _space)| row + 1)
        .sum();
    do_after(grid);
    load
}

fn print_grid(grid: &Grid<Space>) {
    for row in grid.iter_rows() {
        println!("{}", row.map(|space| match space {
            Space::Round => 'O',
            Space::Square => '#',
            Space::Empty => '.',
        }).join(""))
    }
    println!();
}

fn main() {
    let text = fs::read_to_string("14.txt").expect("Error while reading file");

    let mut lines = text.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(Space::from_char).collect(), width
    );

    let total_spins = 1_000_000_000;
    print_grid(&grid);
    let mut history = vec![];
    let mut tracking = None;
    let mut cycle_length = 0;
    let mut ending_i = None;
    for i in tqdm(0..total_spins) {
        if let Some(ref t) = tracking {
            cycle_length += 1;
            if grid == *t {
                println!("Found tracked snapshot again with cycle length {cycle_length}");
                ending_i = Some(i);
                break;
            }
        }
        if tracking.is_none() && history.contains(&grid) {
            println!("Tracking snapshot");
            tracking = Some(grid.clone());
        }
        history.push(grid.clone());
        let old_grid = grid.clone();
        spin_cycle(&mut grid);
        if old_grid == grid {
            println!("Nothing changed after spin cycle ({i})");
        }
    }
    let spin_cycles_to_go = total_spins - (ending_i.unwrap());
    let spin_cycles_to_go_modded = spin_cycles_to_go % cycle_length;
    for i in 0..spin_cycles_to_go_modded {
        spin_cycle(&mut grid);
    }
    let load = get_load(&mut grid, SOUTH);
    println!("{load}");
}

use grid::Grid;
use itertools::Itertools;
use advent_of_code::{Coord, Vector};

advent_of_code::solution!(6);

#[derive(PartialEq, Clone)]
enum Space {
    Empty(bool),
    Guard(Vector),
    Obstruction,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty(false),
            '^' => Self::Guard(Vector::up()),
            '#' => Self::Obstruction,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(lines.join("").chars().map(Space::from_char).collect(), width);
    
    fn step(grid: &mut Grid<Space>) -> bool{
        let ((i, j), g) = grid
            .indexed_iter()
            .find(|((i, j), x)| matches!(x, Space::Guard(_)))
            .unwrap();
        let coord = Coord(i as i32, j as i32);
        match g {
            Space::Guard(vector) => {
                let new_coord = coord + *vector;
                match new_coord.get(grid) {
                    Some(Space::Obstruction) => {
                        let new_vector = vector.turn_90();
                        grid[(i, j)] = Space::Guard(new_vector);
                    }
                    Some(Space::Empty(_)) => {
                        let vector = vector.clone();
                        grid[(i, j)] = Space::Empty(true);
                        grid[new_coord.val()] = Space::Guard(vector);
                    }
                    Some(Space::Guard(_)) => unreachable!(),
                    None => {
                        grid[(i, j)] = Space::Empty(true);
                        return false;
                    }
                }
            }
            _ => unreachable!(),
        }
        true
    }

    while step(&mut grid) {}
    Some(grid.iter().filter(|x| matches!(x, Space::Empty(true))).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(lines.join("").chars().map(Space::from_char).collect(), width);
    fn step(grid: &mut Grid<Space>) -> bool{
        let ((i, j), g) = grid
            .indexed_iter()
            .find(|((i, j), x)| matches!(x, Space::Guard(_)))
            .unwrap();
        let coord = Coord(i as i32, j as i32);
        match g {
            Space::Guard(vector) => {
                let new_coord = coord + *vector;
                match new_coord.get(grid) {
                    Some(Space::Obstruction) => {
                        let new_vector = vector.turn_90();
                        grid[(i, j)] = Space::Guard(new_vector);
                    }
                    Some(Space::Empty(_)) => {
                        let vector = vector.clone();
                        grid[(i, j)] = Space::Empty(true);
                        grid[new_coord.val()] = Space::Guard(vector);
                    }
                    Some(Space::Guard(_)) => unreachable!(),
                    None => {
                        grid[(i, j)] = Space::Empty(true);
                        return false;
                    }
                }
            }
            _ => unreachable!(),
        }
        true
    }
    
    let orig_grid = grid.clone();

    while step(&mut grid) {}
    let visited = grid
        .indexed_iter()
        .filter(|((i, j), x)| matches!(x, Space::Empty(true)));

    let mut count = 0;

    for ((i, j), x) in visited {
        if !matches!(x, Space::Empty(_)) { continue }
        println!("{i},{j}");

        let mut new_grid = orig_grid.clone();
        new_grid[(i, j)] = Space::Obstruction;
        let orig_new_grid = new_grid.clone();

        while step(&mut new_grid) {
            if new_grid == orig_new_grid {
                count += 1;
                continue;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

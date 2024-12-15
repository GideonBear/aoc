use advent_of_code::{Coord, IndexedIterCoord, Vector};
use grid::Grid;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

advent_of_code::solution!(8);

fn get_antinodes(a: Coord, b: Coord) -> [Coord; 2] {
    let vector = a.vector_to(b);
    [a + !vector, b + vector]
}

fn get_antinodes_part<T>(
    coord: Coord,
    vector: Vector,
    grid: &Grid<T>,
) -> impl Iterator<Item = Coord> + use<'_, T> {
    repeat(vector).scan(coord, |state, vector| {
        *state += vector;
        if !state.in_bound(grid) {
            None
        } else {
            Some(*state)
        }
    })
}

fn get_antinodes_two<T>(
    a: Coord,
    b: Coord,
    grid: &Grid<T>,
) -> impl Iterator<Item = Coord> + use<'_, T> {
    let vector = a.vector_to(b);
    let vector_two = !vector;
    get_antinodes_part(a, vector, grid).chain(get_antinodes_part(b, vector_two, grid))
}

enum Space {
    Empty,
    Antenna(char),
}

impl Space {
    fn from_char(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            c => Space::Antenna(c),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let grid = Grid::from_vec(
        lines.join("").chars().map(Space::from_char).collect(),
        width,
    );

    let mut map: HashMap<char, Vec<Coord>> = HashMap::new();

    for (coord, space) in grid.indexed_iter_coord() {
        if let Space::Antenna(c) = space {
            map.entry(*c).or_default().push(coord)
        }
    }

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for (_c, coords) in map {
        for (a, b) in coords
            .into_iter()
            .combinations(2)
            .map(|x| x.into_iter().next_tuple().unwrap())
        {
            for antinode in get_antinodes(a, b) {
                if antinode.in_bound(&grid) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let grid = Grid::from_vec(
        lines.join("").chars().map(Space::from_char).collect(),
        width,
    );

    let mut map: HashMap<char, Vec<Coord>> = HashMap::new();

    for (coord, space) in grid.indexed_iter_coord() {
        if let Space::Antenna(c) = space {
            map.entry(*c).or_default().push(coord)
        }
    }

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for (_c, coords) in map {
        for (a, b) in coords
            .into_iter()
            .combinations(2)
            .map(|x| x.into_iter().next_tuple().unwrap())
        {
            for antinode in get_antinodes_two(a, b, &grid) {
                if antinode.in_bound(&grid) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

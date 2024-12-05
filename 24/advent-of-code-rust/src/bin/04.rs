use grid::Grid;
use itertools::Itertools;
use advent_of_code::{Coord, Vector};

advent_of_code::solution!(4);

fn parse_char(c: char) -> u8 {
    match c {
        'X' => 0,
        'M' => 1,
        'A' => 2,
        'S' => 3,
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().map(parse_char).collect(), width
    );

    let mut count = 0;
    let mut to_print = vec![];
    
    for ((i, j), &x) in grid.indexed_iter() {
        if x != 0 {
            continue;
        }
        for vector in Vector::get_ortho_diagonal() {
            let mut coord = Coord(i.try_into().unwrap(), j.try_into().unwrap());
            for k in 1..4 {
                coord += vector;
                match coord.get(&grid) {
                    None => break,
                    Some(x) => if *x != k { break },
                }
                if k == 3 {
                    count += 1;
                    
                    to_print.push(coord);
                    coord += !vector;
                    to_print.push(coord);
                    coord += !vector;
                    to_print.push(coord);
                    coord += !vector;
                    to_print.push(coord);
                    assert_eq!(coord.0, i32::try_from(i).unwrap());
                    assert_eq!(coord.1, i32::try_from(j).unwrap());
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let mut grid = Grid::from_vec(
        lines.join("").chars().collect(), width
    );

    let mut count = 0;
    // let mut to_print = vec![];

    for ((i, j), &x) in grid.indexed_iter() {
        if x != 'A' {
            continue;
        }
        let mut coord = Coord(i.try_into().unwrap(), j.try_into().unwrap());
        println!("{coord:?}");
        let mut bottom_right = None;
        let mut bottom_left = None;
        for vector in Vector::get_diagonal() {
            println!("{vector:?}");
            let new_coord = coord + vector;
            match new_coord.get(&grid) {
                None => break,
                Some(&new_x) => {
                    let (k, is_last)= match (vector, bottom_right, bottom_left) {
                        (Vector(1, 1), _, _) => (vec!['M', 'S'], false),
                        (Vector(1, -1), _, _) => (vec!['M', 'S'], false),
                        (Vector(-1, -1), Some('M'), _) => (vec!['S'], false),
                        (Vector(-1, -1), Some('S'), _) => (vec!['M'], false),
                        (Vector(-1, 1), _, Some('M')) => (vec!['S'], true),
                        (Vector(-1, 1), _, Some('S')) => (vec!['M'], true),
                        (_, _, _) => unreachable!(),
                    };
                    println!("{k:?}");
                    println!("{new_x:?}");
                    println!("{is_last}");
                    if !k.contains(&new_x) { break }
                    if vector == Vector(1, 1) {
                        bottom_right = Some(new_x);
                    }
                    if vector == Vector(1, -1) {
                        bottom_left = Some(new_x);
                    }
                    if is_last {
                        println!("yay");
                        count += 1;

                        // to_print.push(coord);
                        // coord += !vector;
                        // to_print.push(coord);
                        // coord += !vector;
                        // to_print.push(coord);
                        // coord += !vector;
                        // to_print.push(coord);
                    }
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

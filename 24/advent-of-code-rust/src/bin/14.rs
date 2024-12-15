#![feature(never_type)]

use advent_of_code::{Coord, Vector};
use std::io;
use std::io::{Read, Write};
use std::ops::Range;
use std::str::FromStr;

advent_of_code::solution!(14);

struct Robot {
    position: Coord,
    velocity: Vector,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').unwrap();
        let p = &p[2..];
        let v = &v[2..];

        let (x, y) = p.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        let position = Coord(y, x);

        let (x, y) = v.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        let velocity = Vector(y, x);

        Ok(Self { position, velocity })
    }
}

fn print_robots(robots: &Vec<Robot>, rows: i32, cols: i32) {
    for row in 0..rows {
        for col in 0..cols {
            match robots
                .iter()
                .filter(|x| x.position == Coord(row, col))
                .count()
            {
                0 => print!("."),
                n => print!("{n}"),
            }
        }
        println!();
    }
    println!();
}

fn print_robots_two(robots: &Vec<Robot>, rows: i32, cols: i32) {
    for row in 0..rows {
        for col in 0..cols {
            match robots
                .iter()
                .filter(|x| x.position == Coord(row, col))
                .count()
            {
                0 => print!(" "),
                n => print!("{n}"),
            }
        }
        println!();
    }
    println!();
}

fn print_robots_range(robots: &Vec<Robot>, rows: i32, cols: i32, range: &(Range<i32>, Range<i32>)) {
    for row in 0..rows {
        for col in 0..cols {
            if !(range.0.contains(&row) && range.1.contains(&col)) {
                print!(" ");
                continue;
            }
            match robots
                .iter()
                .filter(|x| x.position == Coord(row, col))
                .count()
            {
                0 => print!("."),
                n => print!("{n}"),
            }
        }
        println!();
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.split("\n").map(|x| x.parse().unwrap()).collect();

    // let mut grid: Grid<Vec<Robot>> = Grid::new(7, 11);
    // let mut grid = Grid::new(103, 101);
    // let (rows, cols) = (7, 11);
    let (rows, cols) = (103, 101);

    print_robots(&robots, rows, cols);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position = robot.position.wrapping_add(robot.velocity, rows, cols);
        }
    }

    print_robots(&robots, rows, cols);

    fn in_range(range: (Range<i32>, Range<i32>), robots: &Vec<Robot>) -> usize {
        robots
            .iter()
            .filter(|x| range.0.contains(&x.position.0) && range.1.contains(&x.position.1))
            .count()
    }

    let top_left = (0..rows / 2, 0..cols / 2);
    print_robots_range(&robots, rows, cols, &top_left);
    let top_left = in_range(top_left, &robots);
    let bottom_left = (rows / 2 + 1..rows, 0..cols / 2);
    print_robots_range(&robots, rows, cols, &bottom_left);
    let bottom_left = in_range(bottom_left, &robots);
    let top_right = (0..rows / 2, cols / 2 + 1..cols);
    print_robots_range(&robots, rows, cols, &top_right);
    let top_right = in_range(top_right, &robots);
    let bottom_right = (rows / 2 + 1..rows, cols / 2 + 1..cols);
    print_robots_range(&robots, rows, cols, &bottom_right);
    let bottom_right = in_range(bottom_right, &robots);

    Some((top_left * bottom_left * top_right * bottom_right) as u32)
}

fn pause() {
    let mut stdin = io::stdin();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.split("\n").map(|x| x.parse().unwrap()).collect();

    // let mut grid: Grid<Vec<Robot>> = Grid::new(7, 11);
    // let mut grid = Grid::new(103, 101);
    // let (rows, cols) = (7, 11);
    let (rows, cols) = (103, 101);

    print_robots(&robots, rows, cols);

    for i in 1..1000000 {
        for robot in robots.iter_mut() {
            robot.position = robot.position.wrapping_add(robot.velocity, rows, cols);
        }
        // print_robots_two(&robots, rows, cols);

        let mut max_size = 0;
        for robot in robots.iter() {
            let mut size = 0;
            let mut had = vec![];
            let mut to_try: Vec<Coord> = Vector::get_ortho()
                .map(|x| Coord::try_from(robot.position).unwrap() + x)
                .collect();

            while !to_try.is_empty() {
                let curr = to_try.pop().unwrap();
                let had_curr = had.clone();
                to_try.extend(
                    Vector::get_ortho()
                        .map(|x| curr + x)
                        .filter(|x| !had_curr.contains(x))
                        .filter(|x| robots.iter().any(|b| b.position == *x))
                        .map(|x| {
                            size += 1;
                            had.push(x);
                            x
                        }),
                )
            }

            max_size = max_size.max(size);
        }

        if max_size > 20 {
            print_robots(&robots, rows, cols);
            println!("{i}");
        }
        print!("\r{i}: {}", max_size);
        io::stdout().flush();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one_easy() {
    //     let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
    //     // assert_eq!(result, Some(12));
    // }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use advent_of_code::{Coord, IndexedIterCoord, Vector};
use grid::Grid;
use itertools::Itertools;
use std::fmt::Display;

advent_of_code::solution!(15);

#[derive(Clone, Copy, PartialEq, Debug)]
enum InitialSpace {
    Space(Space),
    Robot,
}

impl InitialSpace {
    fn from_char(c: char) -> Self {
        match c {
            '@' => InitialSpace::Robot,
            c => InitialSpace::Space(Space::from_char(c)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Space {
    Empty,
    Box,
    Wall,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            'O' => Space::Box,
            '#' => Space::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SpaceTwoTemp {
    Empty,
    Box { is_right: bool },
    Wall,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SpaceTwo {
    Empty,
    Box { is_right: bool, other: Coord },
    Wall,
}

impl Display for SpaceTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceTwo::Empty => write!(f, "."),
            SpaceTwo::Wall => write!(f, "#"),
            SpaceTwo::Box {
                is_right: false,
                other: _,
            } => write!(f, "["),
            SpaceTwo::Box {
                is_right: true,
                other: _,
            } => write!(f, "]"),
        }
    }
}

fn print_grid_two(grid: &Grid<SpaceTwo>, robot: Coord) {
    for (coord, x) in grid.indexed_iter_coord() {
        if coord.1 == 0 {
            println!();
        }
        if coord == robot {
            print!("@");
        } else {
            print!("{x}");
        }
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, movements) = input.split("\n\n").collect_tuple().unwrap();
    let mut lines = map.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let grid = Grid::from_vec(
        lines
            .join("")
            .chars()
            .map(InitialSpace::from_char)
            .collect(),
        width,
    );

    let mut robot = None;
    let mut grid = Grid::from_vec(
        grid.indexed_iter_coord()
            .map(|(coord, &x)| match x {
                InitialSpace::Robot => {
                    robot = Some(coord);
                    Space::Empty
                }
                InitialSpace::Space(space) => space,
            })
            .collect(),
        width,
    );
    let mut robot = robot.unwrap();

    let movements = movements
        .split("\n")
        .map(|x| x.chars())
        .flatten()
        .map(|x| Vector::from_arrow(x));

    for movement in movements {
        let new = robot + movement;
        match new.get(&grid).unwrap() {
            Space::Empty => {
                robot = new;
            }
            Space::Wall => {
                // Do nothing
            }
            Space::Box => {
                let mut curr = new;
                loop {
                    curr += movement;
                    match curr.get(&grid).unwrap() {
                        Space::Wall => {
                            // Cannot move, break and do nothing
                            // println!("Cannot move, break and do nothing.");
                            break;
                        }
                        Space::Box => {
                            // Continue searching for empty space/wall
                            // println!("Next box...");
                        }
                        Space::Empty => {
                            // Found empty spot to push to.
                            // println!("Found empty spot to push to.");
                            // This one should become a box:
                            grid[curr.val()] = Space::Box;
                            // And the robot should replace the first box:
                            grid[new.val()] = Space::Empty;
                            robot = new;
                            // Then we're done.
                            break;
                        }
                    }
                }
            }
        }
    }

    Some(
        grid.indexed_iter_coord()
            .filter(|(_coord, x)| matches!(x, Space::Box))
            .map(|(coord, _x)| 100 * coord.0 + coord.1)
            .map(|x| u32::try_from(x).unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, movements) = input.split("\n\n").collect_tuple().unwrap();
    let mut lines = map.split('\n').peekable();
    let width = lines.peek().unwrap().len();
    let grid = Grid::from_vec(
        lines
            .join("")
            .chars()
            .map(InitialSpace::from_char)
            .collect(),
        width,
    );

    let mut robot = None;
    let grid = Grid::from_vec(
        grid.indexed_iter_coord()
            .map(|(coord, &x)| match x {
                InitialSpace::Robot => {
                    robot = Some(coord);
                    Space::Empty
                }
                InitialSpace::Space(space) => space,
            })
            .collect(),
        width,
    );
    let robot = robot.unwrap();

    let width = width * 2;
    let mut robot = Coord(robot.0, robot.1 * 2);

    let grid = Grid::from_vec(
        grid.iter()
            .flat_map(|x| match x {
                Space::Empty => vec![SpaceTwoTemp::Empty, SpaceTwoTemp::Empty],
                Space::Wall => vec![SpaceTwoTemp::Wall, SpaceTwoTemp::Wall],
                Space::Box => vec![
                    SpaceTwoTemp::Box { is_right: false },
                    SpaceTwoTemp::Box { is_right: true },
                ],
            })
            .collect(),
        width,
    );

    let mut grid = Grid::from_vec(
        grid.indexed_iter_coord()
            .map(|(coord, x)| match x {
                SpaceTwoTemp::Empty => SpaceTwo::Empty,
                SpaceTwoTemp::Wall => SpaceTwo::Wall,
                SpaceTwoTemp::Box { is_right } => {
                    if *is_right {
                        SpaceTwo::Box {
                            is_right: *is_right,
                            other: coord + Vector::left(),
                        }
                    } else {
                        SpaceTwo::Box {
                            is_right: *is_right,
                            other: coord + Vector::right(),
                        }
                    }
                }
            })
            .collect(),
        width,
    );

    let movements = movements
        .split("\n")
        .map(|x| x.chars())
        .flatten()
        .map(|x| Vector::from_arrow(x));

    fn move_box(space: SpaceTwo, vector: Vector) -> SpaceTwo {
        match space {
            SpaceTwo::Box { is_right, other } => SpaceTwo::Box {
                is_right,
                other: other + vector,
            },
            space => space,
        }
    }

    fn push(
        grid: &mut Grid<SpaceTwo>,
        coord: Coord,
        vector: Vector,
        indent: usize,
        really_do: bool,
    ) -> bool {
        let new = coord + vector;
        // print!("{}", " ".repeat(indent));
        // println!("I am a {} at {coord} and I need to push {vector} towards a {}", coord.get(&grid).unwrap(), new.get(&grid).unwrap());
        match new.get(&grid.clone()).unwrap() {
            SpaceTwo::Empty => {
                // If it's the robot it just replaces empty with empty
                if really_do {
                    grid[new.val()] = move_box(grid[coord.val()], vector);
                }
                true
            }
            SpaceTwo::Wall => {
                // Do nothing
                false
            }
            SpaceTwo::Box { is_right: _, other } => {
                if [Vector::up(), Vector::down()].contains(&vector) {
                    // print!("{}", " ".repeat(indent));
                    // println!("  The thing I'm pushing against is a box, so this will push two things: {} and {}", new, *other);
                    let a = push(grid, new, vector, indent + 4, false);
                    let c = if a {
                        // print!("{}", " ".repeat(indent));
                        // println!("  Pushing the second now.");
                        let b = push(grid, *other, vector, indent + 4, really_do);
                        if b {
                            push(grid, new, vector, indent + 4, really_do);
                        }
                        b
                    } else {
                        false
                    };
                    if c {
                        // print!("{}", " ".repeat(indent));
                        // println!("  Now actually doing the push of both.");
                        if really_do {
                            grid[new.val()] = move_box(grid[coord.val()], vector);
                            grid[other.val()] = move_box(SpaceTwo::Empty, vector);
                        }
                        // print_grid_two(&grid, Coord(0, 0));
                    }
                    c
                } else {
                    // print!("{}", " ".repeat(indent));
                    // println!("Horizontal push.");
                    if push(grid, new, vector, indent + 4, really_do) {
                        if really_do {
                            grid[new.val()] = move_box(grid[coord.val()], vector);
                        }
                        true
                    } else {
                        false
                    }
                }

                // let mut curr = new;
                // loop {
                //     curr += vector;
                //     match curr.get(&grid).unwrap() {
                //         SpaceTwo::Wall => {
                //             // Cannot move, break and do nothing
                //             // println!("Cannot move, break and do nothing.");
                //             break false;
                //         }
                //         SpaceTwo::Box => {
                //             // Continue searching for empty space/wall
                //             // println!("Next box...");
                //         }
                //         SpaceTwo::Empty => {
                //             // Found empty spot to push to.
                //             // println!("Found empty spot to push to.");
                //             // This one should become a box:
                //             grid[curr.val()] = SpaceTwo::Box;
                //             // And the robot should replace the first box:
                //             grid[new.val()] = SpaceTwo::Empty;
                //             // Then we're done.
                //             break true;
                //         }
                //     }
                // }
            }
        }
    }

    // print_grid_two(&grid, robot);
    for movement in movements {
        if push(&mut grid, robot, movement, 0, true) {
            robot += movement;
        }
        // print_grid_two(&grid, robot);
    }

    Some(
        grid.indexed_iter_coord()
            .filter(|(_coord, x)| {
                matches!(
                    x,
                    SpaceTwo::Box {
                        other: _,
                        is_right: false
                    }
                )
            })
            .map(|(coord, _x)| 100 * coord.0 + coord.1)
            .map(|x| u32::try_from(x).unwrap())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_easy() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_easy() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}

//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! ```

use std::fs;
use grid::*;
use itertools::Itertools;

fn parse_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!(),
    }
}

fn find_mirrors(grid: &Grid<bool>) -> Vec<usize> {
    let mut mirrors = vec![];
    for mirror in 1..grid.rows() {
        let rows: Vec<Vec<_>> = grid.iter_rows().map(|x| x.collect()).collect();
        let mut front = rows[..mirror].to_vec();
        let mut back = rows[mirror..].to_vec();
        if front.len() > back.len() {
            front.reverse();
            front = (&front[..back.len()]).to_vec();
        } else if back.len() > front.len() {
            back = (&back[..front.len()]).to_vec();
            back.reverse()
        } else {
            back.reverse();
        }
        // println!("Checking {mirror}: {:?}", front == back);
        // if mirror == 5 {
        //     println!("{:?} == {:?}", &front, &back);
        // }
        if front == back {
            println!("  - Found mirror at {mirror}");
            mirrors.push(mirror);
        }
    }
    if mirrors.len() > 1 {
        println!("  ! MORE THAN ONE MIRROR FOUND");
    }
    mirrors
}

fn main() {
    let text = fs::read_to_string("13.txt").expect("Error while reading file");

    let patterns = text.split("\n\n").map(|subtext| {
        let mut lines = subtext.split('\n').peekable();
        let width = lines.peek().unwrap().len();
        Grid::from_vec(
            lines.join("").chars().map(parse_char).collect(), width
        )
    });

    let result: usize = patterns.enumerate().map(|(i, mut grid)| {
        let mut total = 0;
        println!("Pat {i}");
        println!("- Checking pat horizontal");
        for mirror in find_mirrors(&grid) {
            total += mirror * 100
        }
        println!("- Checking pat vertical");
        grid.transpose();
        for mirror in find_mirrors(&grid) {
            total += mirror
        }
        if total == 0 {
            println!("- No mirror found for pat {i}");
        }
        total
    }).sum();

    println!("{result}");
}

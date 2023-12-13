//! ```cargo
//! [dependencies]
//! grid = "0.12.0"
//! itertools = "0.12.0"
//! tupleops = "0.1.1"
//! ```

use std::fs;
use grid::*;
use itertools::Itertools;
use tupleops::ref_tuple;

fn iter_all_eq<T: PartialEq>(mut iter: impl Iterator<Item = T>) -> Option<T> {
    let first = iter.next()?;
    iter.all(|elem| elem == first).then(|| first)
}

fn parse_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!(),
    }
}

fn find_mirrors(grid: &Grid<bool>, mirror_to_skip: Option<&Vec<usize>>) -> Vec<usize> {
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
    if let Some(mirror_to_skip) = mirror_to_skip {
        mirrors = mirrors.into_iter().filter(|x| {
            if mirror_to_skip.contains(x) {
                println!("Nevermind; skipping {x}");
                false
            } else { true }
        }).collect();
    }
    if mirrors.len() > 1 {
        println!("  ! MORE THAN ONE MIRROR FOUND");
        match iter_all_eq(mirrors.iter()) {
            Some(x) => {
                println!("  ! ALL ARE THE SAME, REPLACING ALL WITH ONE");
                mirrors = vec![*x];
            }
            None => {
                println!("  ! ALL ARE NOT THE SAME, PANICKING");
                panic!("{:?}", mirrors);
            }
        }
    }
    mirrors
}

fn find_mirrors_both_ways(mut grid: Grid<bool>, mirror_to_skip: Option<&(Vec<usize>, Vec<usize>)>) -> (Vec<usize>, Vec<usize>) {
    let mut horizontal_mirrors = vec![];
    // println!("- Checking pat horizontal");
    for mirror in find_mirrors(&grid, mirror_to_skip.map(|x| ref_tuple(x).0)) {
        horizontal_mirrors.push(mirror)
    }
    let mut vertical_mirrors = vec![];
    // println!("- Checking pat vertical");
    grid.transpose();
    for mirror in find_mirrors(&grid, mirror_to_skip.map(|x| ref_tuple(x).1)) {
        vertical_mirrors.push(mirror)
    }
    (horizontal_mirrors, vertical_mirrors)
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

    let result: usize = patterns.enumerate().map(|(i, grid)| {
        println!("Pat {i}");
        let mut total = 0;
        let mut done = vec![];
        let originals = find_mirrors_both_ways(grid.clone(), None);
        for (coords, original) in grid.indexed_iter() {
            let mut new_grid = grid.clone();
            new_grid[coords] = !original;
            let (horizontal, vertical) = find_mirrors_both_ways(new_grid, Some(&originals));
            total += horizontal.into_iter().filter(|x| {
                if done.contains(x) {
                    false
                } else {
                    done.push(*x);
                    true
                }
            }).sum::<usize>() * 100;
            total += vertical.into_iter().filter(|x| {
                if done.contains(x) {
                    false
                } else {
                    done.push(*x);
                    true
                }
            }).sum::<usize>();
        }
        if total == 0 {
            println!("- NO MIRROR FOUND FOR PAT {i}");
        }
        total
    }).sum();

    println!("{result}");
}

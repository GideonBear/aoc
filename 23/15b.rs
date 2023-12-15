//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! peeking_take_while = "1.0.0"
//! ```

use std::fs;
//use itertools::Itertools;
use peeking_take_while::PeekableExt;

fn hash(s: &String) -> u8 {
    let mut curr = 0;
    for char in s.chars() {
        let ascii = char as u8;
        curr += ascii;
        curr *= 17;
    }
    curr
}

struct Step {
    label: String,
    op: StepOp,
}

enum StepOp {
    Remove,
    Put(u8),
}

impl Step {
    fn from_string(s: &str) -> Self {
        let mut it = s.chars().peekable();
        let label = it
            .peeking_take_while(|&x| x != '=' && x != '-')
            .collect();
        let op = match it.next().unwrap() {
            '-' => StepOp::Remove,
            '=' => StepOp::Put(it.collect::<String>().parse().unwrap()),
            c => panic!("{c} in {s}"),
        };
        Self { label, op }
    }
}

fn main() {
    let text = fs::read_to_string("15.txt").expect("Error while reading file");

    let steps = text
        .split(',')
        .map(Step::from_string);

    const BOX_DEFAULT: Vec<(String, u8)> = vec![];
    let mut boxes: [Vec<(String, u8)>; 256] = [BOX_DEFAULT; 256];
    for step in steps {
        let hash = hash(&step.label);
        let b = &mut boxes[usize::from(hash)];
        match step.op {
            StepOp::Remove => {
                for (i, (blabel, _blens)) in b.iter().enumerate() {
                    if blabel == &step.label {
                        b.remove(i);
                        break;
                    }
                }
            }
            StepOp::Put(lens) => {
                let mut should_add = true;
                for (i, (blabel, _blens)) in b.iter().enumerate() {
                    if blabel == &step.label {
                        b[i].1 = lens;
                        should_add = false;
                        break;
                    }
                }
                if should_add {
                    b.push((step.label, lens));
                }
            }
        }
        println!("After ");
        for (i, b) in boxes.iter().enumerate().filter(|(_i, b)| !b.is_empty()) {
            println!("Box {i}: {b:?}");
        }
        println!();
    }

    let result = boxes
        .into_iter()
        .enumerate()
        .map(|(bi, b)| {
            b.into_iter().enumerate().map(|(li, (label, lens))| {
                (1 + bi)
                * (1 + li)
                * (lens as usize)
            }).sum::<usize>()
        })
        .sum::<usize>();

    println!("{result}");
}

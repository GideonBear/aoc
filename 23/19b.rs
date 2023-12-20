//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use std::ops::Range;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Cat {
    X,
    M,
    A,
    S,
}

impl From<char> for Cat {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<(Condition, SendTo)>,
    fallback: SendTo,
}

impl Workflow {
    fn from_string(s: &str) -> (String, Self) {
        let (name, rules) = s.split('{').collect_tuple().unwrap();
        let mut rules = rules.strip_suffix('}').unwrap().split(',');
        let fallback = rules.next_back().unwrap().into();
        let rules = rules.map(|rule| {
            let (condition, send_to) = rule.split(':').collect_tuple().unwrap();
            (condition.into(), send_to.into())
        }).collect();

        (name.to_string(), Self { rules, fallback })
    }
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    cat: Cat,
    op: Op,
    val: u32,
}

impl From<&str> for Condition {
    fn from(s: &str) -> Self {
        let mut it = s.chars();
        let cat = it.next().unwrap().into();
        let op = it.next().unwrap().into();
        let val = it.collect::<String>().parse().unwrap();
        Self { cat, op, val }
    }
}

impl Condition {
    fn apply(&self, part: &Part) -> (Part, Part) {
        // self.op.apply(part.ratings[&self.cat], self.val)
        let range = &part.ratings[&self.cat];
        let add = if let Op::GT = self.op { 1 } else { 0 };
        let small_part_range = range.start .. self.val + add;
        let big_part_range = self.val + add .. range.end;
        let mut small_part = Part { ratings: part.ratings.clone() };
        let mut big_part = Part { ratings: part.ratings.clone() };
        small_part.ratings.insert(self.cat, small_part_range);
        big_part.ratings.insert(self.cat, big_part_range);
        match self.op {
            Op::LT => (small_part, big_part),
            Op::GT => (big_part, small_part),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    LT,
    GT,
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::LT,
            '>' => Self::GT,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum SendTo {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for SendTo {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Workflow(s.to_string()),
        }
    }
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<Cat, Range<u32>>,
}

impl Part {
    fn new() -> Self {
        Self {
            ratings: HashMap::from([
                (Cat::X, 1..4001),
                (Cat::M, 1..4001),
                (Cat::A, 1..4001),
                (Cat::S, 1..4001),
            ])
        }
    }

    fn combs(&self) -> u128 {
        println!("Accepted {self:?}");
        // println!("{}", self
        //     .ratings
        //     .values()
        //     .map(|x| u128::try_from(x.len()).unwrap())
        //     .product::<u128>());
        self
            .ratings
            .values()
            .map(|x| u128::try_from(x.len()).unwrap())
            .product()
    }
}

fn main() {
    let text = fs::read_to_string("19.txt").expect("Error while reading file");

    let (workflows, _parts) = text.split("\n\n").collect_tuple().unwrap();
    let workflows: HashMap<String, Workflow> = workflows.split('\n').map(Workflow::from_string).collect();
    // let workflow_in = &workflows["in"];
    let full_part = Part::new();

    let mut total = 0;
    let send_to_in = SendTo::Workflow("in".to_string());
    let mut todo = vec![(full_part, &send_to_in)];
    while !todo.is_empty() {
        let (mut part, send_to) = todo.pop().unwrap();
        match send_to {
            SendTo::Accept => total += part.combs(),
            SendTo::Reject => (),
            SendTo::Workflow(s) => {
                println!("Going through workflow {s}");
                let workflow = &workflows[s];
                for (condition, this_send_to) in &workflow.rules {
                    println!("- {this_send_to:?} : {condition:?}");
                    let (accepted, rejected) = condition.apply(&part);
                    todo.push((accepted, this_send_to));
                    part = rejected;
                }
                println!("- {:?} : fallback", workflow.fallback);
                todo.push((part, &workflow.fallback));
            }
        }
    }

    println!("{total}");
}

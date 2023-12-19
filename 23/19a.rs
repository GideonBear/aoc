//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

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
    fn apply(&self, part: &Part) -> bool {
        self.op.apply(part.ratings[&self.cat], self.val)
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

impl Op {
    fn apply<T: std::cmp::Ord>(&self, a: T, b: T) -> bool {
        match self {
            Self::LT => a < b,
            Self::GT => a > b,
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
    ratings: HashMap<Cat, u32>,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        Self {
            ratings: s
                .split(',')
                .map(|r| {
                    let mut it = r.chars();
                    let cat = it.next().unwrap().into();
                    it.next();
                    let val = it.collect::<String>().parse().unwrap();
                    (cat, val)
                })
                .collect()
        }
    }
}

fn process_part(part: &Part, workflows: &HashMap<String, Workflow>, workflow_in: &Workflow) -> bool {
    let mut curr = workflow_in;
    loop {
        let mut send_to = &curr.fallback;
        for (condition, this_send_to) in &curr.rules {
            if condition.apply(part) {
                send_to = this_send_to;
                break;
            }
        }
        match send_to {
            SendTo::Accept => return true,
            SendTo::Reject => return false,
            SendTo::Workflow(wf) => curr = &workflows[wf],
        }
    }
}

fn main() {
    let text = fs::read_to_string("19.txt").expect("Error while reading file");

    let (workflows, parts) = text.split("\n\n").collect_tuple().unwrap();
    let workflows: HashMap<String, Workflow> = workflows.split('\n').map(Workflow::from_string).collect();
    let workflow_in = &workflows["in"];
    let parts = parts.split('\n').map(Part::from);

    let result = parts
        .filter(|part| process_part(part, &workflows, workflow_in))
        .map(|part| part.ratings.into_values().sum::<u32>())
        .sum::<u32>();

    println!("{result}");
}

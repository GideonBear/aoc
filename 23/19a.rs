use std::fs;

enum Cat {
    X,
    M,
    A,
    S,
}

impl Cat {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!(),
        }
    }
}

struct Workflow {
    rules: Vec<(Condition, SendTo)>,
    fallback: SendTo,
}

impl Workflow {
    fn from_string(s: &str) -> Self {
        todo!()
    }
}

struct Condition {
    cat: Cat,
    op: Op,
    val: u32,
}

impl Condition {
    fn from_string(s: &str) -> Self {
        let it = s.chars();
        let cat = Cat::from_char(it.next());
        let op = Op::from_char(it.next());
        let val = it.collect::<String>().parse().unwrap();
        Self { cat, op, val }
    }
}

enum Op {
    LT,
    GT,
}

impl Op {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::LT,
            '>' => Self::GT,
            _ => panic!(),
        }
    }

    fn apply<T>(&self, a: T, b: T) -> bool {
        match self {
            Self::LT => a < b,
            Self::GT => a > b,
        }
    }
}

enum SendTo {
    Accept,
    Reject,
    Workflow(String),
}

impl SendTo {
    fn from_string(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Workflow(s),
        }
    }
}

struct Part {
    ratings: HashMap<Cat, u32>,
}

impl Part {
    fn from_string(s: &str) -> Self {
        todo!()
    }
}

fn main() {
    let text = fs::read_to_string("19.txt").expect("Error while reading file");

    let (workflows, parts) = text.split("\n\n");
    let workflows = workflows.map(Workflow::from_string);
    let parts = parts.map(Part::from_string);
}

//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

struct Module {
    dests: Vec<String>,
    module_type: ModuleType,
}

impl Module {
    fn new(module_type: char, dests: Vec<String>) -> Self {
        Self {
            dests, module_type: ModuleType::new(module_type)
        }
    }
}

enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

impl ModuleType {
    fn new(module_type: char) -> Self {
        match module_type {
            '%' => Self::FlipFlop(false),
            '&' => Self::Conjunction(HashMap::new()),
            'b' => Self::Broadcaster,
            _ => panic!(),
        }
    }

    fn rcv(&mut self, pulse: bool, sender: String) -> Option<bool> {
        match self {
            Self::FlipFlop(state) => {
                if !pulse {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            Self::Conjunction(memory) => {
                *memory.entry(sender).or_insert(false) = pulse;
                Some(!memory.values().all(|&p| p))
            }
            Self::Broadcaster => {
                Some(pulse)
            }
        }
    }
}


fn main() {
    let text = fs::read_to_string("20.txt").expect("Error while reading file");

    let modules: HashMap<String, Module> = text
        .split('\n')
        .map(|line| line.split(" -> ").collect_tuple().unwrap())
        .map(|(name, dests)| {
            let dests = dests.split(", ").map(|x| x.to_string()).collect();
            let mut name_it = name.chars();
            let module = Module::new(name_it.next().unwrap(), dests);
            let name = name_it.collect();
            (name, module)
        })
        .collect();

    todo!();
}

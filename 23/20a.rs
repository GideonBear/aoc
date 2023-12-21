//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn pulse_name(pulse: bool) -> &'static str {
    match pulse {
        true => "high",
        false => "low",
    }
}

struct Module {
    name: String,
    dests: Vec<String>,
    module_type: ModuleType,
}

impl Module {
    fn new(name: String, module_type: char, dests: Vec<String>) -> Self {
        Self {
            name,
            dests,
            module_type: ModuleType::new(module_type)
        }
    }

    fn rcv(&mut self, pulse: bool, sender: String) -> Option<impl Iterator<Item = (String, bool)> + '_> {
        let name = &self.name;
        match self.module_type.rcv(pulse, sender) {
            Some(new_pulse) => Some(self.dests.iter().map(move |dest| {
                println!("{} -{}-> {}", name, pulse_name(new_pulse), dest);
                (dest.clone(), new_pulse)
            })),
            None => None
        }
    }
}

enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
    Untyped,
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
                    // println!("Flipflop activated, initial state {state}, flipping to and sending {}", !*state);
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            Self::Conjunction(memory) => {
                // TODO: get list of all parent modules passed into Module::new and populate memory in advance
                // println!("Conjunction activated");
                *memory.entry(sender).or_insert(false) = pulse;
                Some(!memory.values().all(|&p| p))
            }
            Self::Broadcaster => Some(pulse),
            Self::Untyped => None,
        }
    }
}

fn press_button(modules: &mut HashMap<String, Module>) {
    let mut this_round = vec![("roadcaster".to_string(), false, "button".to_string())];
    let mut next_round = vec![];

    while !this_round.is_empty() {
        for (module_name, pulse, sender) in this_round {
            let module = modules
                .entry(module_name.clone())
                .or_insert_with(|| Module {
                    name: module_name.clone(),
                    dests: vec![],
                    module_type: ModuleType::Untyped,
                });
            next_round.extend(
                module.rcv(pulse, sender)
                    .into_iter().flatten() // None becomes empty iterator
                    .map(|(n, p)| (n, p, module_name.clone()))
            );
        }
        this_round = next_round;
        next_round = vec![];
    }
    println!();
}

fn main() {
    let text = fs::read_to_string("20e2.txt").expect("Error while reading file");

    let mut modules: HashMap<String, Module> = text
        .split('\n')
        .map(|line| line.split(" -> ").collect_tuple().unwrap())
        .map(|(name, dests)| {
            let dests = dests.split(", ").map(|x| x.to_string()).collect();
            let mut name_it = name.chars();
            let module_type = name_it.next().unwrap();
            let name: String = name_it.collect();
            let module = Module::new(name.clone(), module_type, dests);
            (name, module)
        })
        .collect();

    press_button(&mut modules);
    press_button(&mut modules);
}

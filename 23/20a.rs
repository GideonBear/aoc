//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! tqdm = "0.6.0"
//! ```

use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn merge<K: std::hash::Hash + std::cmp::Eq, V>(a: HashMap<K, V>, b: HashMap<K, V>, op: impl Fn (V, &V) -> V) -> HashMap<K, V> {
    a
        .into_iter()
        .map(|(k, v)| {
            let v2 = &b[&k];
            (k, op(v, v2))
        })
        .collect()
}

fn apply<K: std::hash::Hash + std::cmp::Eq, V>(a: HashMap<K, V>, op: impl Fn (V) -> V) -> HashMap<K, V> {
    a
        .into_iter()
        .map(|(k, v)| (k, op(v)))
        .collect()
}

fn pulse_name(pulse: bool) -> &'static str {
    match pulse {
        true => "high",
        false => "low",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
                // println!("{} -{}-> {}", name, pulse_name(new_pulse), dest);
                (dest.clone(), new_pulse)
            })),
            None => None
        }
    }
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
                // println!("Conjunction activated");
                memory.insert(sender, pulse);
                Some(!memory.values().all(|&p| p))
            }
            Self::Broadcaster => Some(pulse),
            Self::Untyped => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
    Untyped,
}

fn push_button(modules: &mut HashMap<String, Module>) -> HashMap<bool, usize> {
    let mut count = HashMap::from([
        (true, 0),
        (false, 0),
    ]);
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
            *count.get_mut(&pulse).unwrap() += 1;
            next_round.extend(
                module.rcv(pulse, sender)
                    .into_iter().flatten() // None becomes empty iterator
                    .map(|(n, p)| (n, p, module_name.clone()))
            );
        }
        this_round = next_round;
        next_round = vec![];
    }
    // println!();
    count
}

fn main() {
    let text = fs::read_to_string("20.txt").expect("Error while reading file");

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
    let modules_clone = modules.clone();

    // Prepare conjunction modules
    for (name, module) in &mut modules {
        match &mut module.module_type {
            ModuleType::Conjunction(memory) => {
                for module_j in modules_clone.values().filter(|x| x.dests.contains(&name)) {
                    memory.insert(module_j.name.clone(), false);
                }
            }
            _ => (),
        }
    }

    let mut history = vec![modules.clone()];
    let mut count = push_button(&mut modules);
    loop {
        history.push(modules.clone());
        println!("Step {}", history.len());
        let this_count = push_button(&mut modules);
        if history.contains(&modules) || history.len() == 1001 {
            break;
        }
        count = merge(count, this_count, |a, b| a + b);
    }
    let cycle_length = history.len() - 1;
    println!("Cycle length: {cycle_length:?}");
    println!("Cycle count: {count:?}");
    count = apply(count, |v| v * (1000 / cycle_length));
    for i in 0..(1000 % cycle_length) {
        let this_count = push_button(&mut modules);
        count = merge(count, this_count, |a, b| a + b);
    }
    println!("Total count: {count:?}");

    let result = count[&true] * count[&false];
    println!("{result}")
}

use std::fs;

fn main() {
    let text = fs::read_to_string("13.txt").expect("Error while reading file");
}


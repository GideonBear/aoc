use std::fs;

fn main() {
    let text = fs::read_to_string("23.txt").expect("Error while reading file");
}



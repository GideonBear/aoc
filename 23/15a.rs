use std::fs;

fn hash(s: &str) -> u8 {
    let mut curr = 0;
    for char in s.chars() {
        let ascii = char as u8;
        curr += ascii;
        curr *= 17;
    }
    curr
}

fn main() {
    let text = fs::read_to_string("15.txt").expect("Error while reading file");

    let steps = text.split(',');
    let result: u32 = steps.map(|x| u32::from(hash(x))).sum();
    println!("{result}");
}

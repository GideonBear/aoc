//! ```cargo
//! [dependencies]
//! itertools = "0.12.0"
//! ```

use std::fs;
use itertools::Itertools;

struct Ma {
    from_cat: String,
    to_cat: String,
    ranges: Vec<(u16, u16, u16)>,
}

impl Ma {
    fn from_string(from_cat: String, to_cat: String, string: String) -> Self {
        return Self {
            from_cat,
            to_cat,
            ranges: string.split('\n')
                .map(|ma_range| {
                    ma_range
                        .split(' ')
                        .map(|x| x.parse().expect("Should be a number"))
                        .collect_tuple()
                        .expect("Number of items incorrect")
                })
                .collect(),
        }
    }
}

fn main() {
    let text = fs::read_to_string("5.txt").expect("Error while reading file");


}

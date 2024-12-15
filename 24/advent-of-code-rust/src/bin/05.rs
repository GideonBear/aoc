use itertools::Itertools;

advent_of_code::solution!(5);

fn get_middle<T>(l: &[T]) -> &T {
    l.get(l.len() / 2).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = input.split("\n\n").next_tuple().unwrap();
    let rules: Vec<(u32, u32)> = rules
        .split("\n")
        .map(|x| {
            x.split("|")
                .map(|x| x.parse().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect();
    let updates: Vec<Vec<u32>> = updates
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    Some(
        updates
            .iter()
            .filter(|update| {
                for (a, b) in rules.iter() {
                    if update.contains(a) && update.contains(b) {
                        let ia = update.iter().position(|x| x == a);
                        let ib = update.iter().position(|x| x == b);
                        if ia > ib {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|x| get_middle(x))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = input.split("\n\n").next_tuple().unwrap();
    let rules: Vec<(u32, u32)> = rules
        .split("\n")
        .map(|x| {
            x.split("|")
                .map(|x| x.parse().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect();
    let updates: Vec<Vec<u32>> = updates
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    Some(
        updates
            .iter()
            .filter_map(|update| {
                let mut new_update = update.clone();
                let mut did_something = true;
                while did_something {
                    did_something = false;
                    for (a, b) in rules.iter() {
                        if new_update.contains(a) && new_update.contains(b) {
                            let ia = new_update.iter().position(|x| x == a).unwrap();
                            let ib = new_update.iter().position(|x| x == b).unwrap();
                            if ia > ib {
                                new_update.swap(ia, ib);
                                did_something = true;
                            }
                        }
                    }
                }
                if *update == new_update {
                    None
                } else {
                    Some(new_update)
                }
            })
            .map(|l| *get_middle(&l))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

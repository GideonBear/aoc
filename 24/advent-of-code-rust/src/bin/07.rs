use itertools::Itertools;
use std::iter::repeat_n;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u64>)> = input
        .split("\n")
        .map(|l| l.split(": ").next_tuple().unwrap())
        .map(|(value, operands)| {
            (
                value.parse().unwrap(),
                operands.split(" ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect();
    Some(
        equations
            .into_iter()
            .filter_map(|(value, operands)| {
                let first = *operands.first().unwrap();
                let choices = operands.len() - 1;
                assert_eq!(2_u64.pow(choices as u32), repeat_n(['*', '+'].into_iter(), choices).multi_cartesian_product().count() as u64);
                // dbg!(repeat_n(['*', '+'].into_iter(), choices).multi_cartesian_product().collect::<Vec<_>>());
                for permutation in
                    repeat_n(['*', '+'].into_iter(), choices).multi_cartesian_product()
                {
                    let mut current = first;
                    for (i, operand) in operands.iter().skip(1).enumerate() {
                        current = match permutation[i] {
                            '*' => current.checked_mul(*operand)?,
                            '+' => current + operand,
                            _ => unreachable!(),
                        }
                    }
                    if u64::from(current) == value {
                        return Some(value);
                    }
                }
                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u64>)> = input
        .split("\n")
        .map(|l| l.split(": ").next_tuple().unwrap())
        .map(|(value, operands)| {
            (
                value.parse().unwrap(),
                operands.split(" ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect();
    Some(
        equations
            .into_iter()
            .filter_map(|(value, operands)| {
                let first = *operands.first().unwrap();
                let choices = operands.len() - 1;
                assert_eq!(3_u64.pow(choices as u32), repeat_n(['*', '+', '|'].into_iter(), choices).multi_cartesian_product().count() as u64);
                // dbg!(repeat_n(['*', '+'].into_iter(), choices).multi_cartesian_product().collect::<Vec<_>>());
                for permutation in
                    repeat_n(['*', '+', '|'].into_iter(), choices).multi_cartesian_product()
                {
                    let mut current = first;
                    for (i, operand) in operands.iter().skip(1).enumerate() {
                        current = match permutation[i] {
                            '*' => current.checked_mul(*operand)?,
                            '+' => current + operand,
                            '|' => (current.to_string() + &operand.to_string()).parse().ok()?,
                            _ => unreachable!(),
                        }
                    }
                    if u64::from(current) == value {
                        return Some(value);
                    }
                }
                None
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

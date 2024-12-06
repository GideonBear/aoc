use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];
    for line in input.split("\n") {
        let (v1, v2) = line
            .split("   ")
            .map(str::parse)
            .map(Result::unwrap)
            .next_tuple()
            .unwrap();
        l1.push(v1);
        l2.push(v2);
    }
    l1.sort();
    l2.sort();
    Some(
        l1.into_iter()
            .zip(l2)
            .map(|(v1, v2)| (v1 - v2).unsigned_abs() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut l1: Vec<usize> = vec![];
    let mut l2: Vec<usize> = vec![];
    for line in input.split("\n") {
        let (v1, v2) = line
            .split("   ")
            .map(str::parse)
            .map(Result::unwrap)
            .next_tuple()
            .unwrap();
        l1.push(v1);
        l2.push(v2);
    }
    l1.sort();
    l2.sort();

    let mut sim = 0;
    for v1 in l1 {
        let count = l2.iter().filter(|&x| *x == v1).count();
        sim += v1 * count;
    }
    Some(sim.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

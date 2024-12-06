use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    Some(
        re.captures_iter(input)
            .map(|cap| cap.extract())
            .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let re = Regex::new(r"(do)\(\)()()|(don't)\(\)()()|(mul)\(([0-9]+),([0-9]+)\)").unwrap();

    Some(
        re.captures_iter(input)
            .map(|cap| cap.extract())
            .map(|(_, [name, a, b])| {
                if name == "do" {
                    enabled = true;
                    0
                } else if name == "don't" {
                    enabled = false;
                    0
                } else {
                    if enabled {
                        a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
                    } else {
                        0
                    }
                }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}

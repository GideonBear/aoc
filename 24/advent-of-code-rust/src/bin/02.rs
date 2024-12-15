advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .split("\n")
        .map(|r| r.split(" "))
        .map(|r| r.map(|l| l.parse::<u32>().unwrap()).collect())
        .collect();
    let mut safe = 0;
    'outer: for report in reports {
        let first = report.first().unwrap();
        let second = report.get(1).unwrap();
        let allowed = if second > first {
            [1, 2, 3]
        } else if second < first {
            [-1, -2, -3]
        } else {
            continue;
        };
        for (l1, l2) in report.windows(2).map(|w| (w[0], w[1])) {
            if !allowed.contains(&(i32::try_from(l2).unwrap() - i32::try_from(l1).unwrap())) {
                continue 'outer;
            }
        }
        safe += 1;
    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .split("\n")
        .map(|r| r.split(" "))
        .map(|r| r.map(|l| l.parse::<u32>().unwrap()).collect())
        .collect();
    let mut safe = 0;
    for o_report in reports {
        'outer: for i in 0..(o_report.len() + 1) {
            let mut report = o_report.clone();
            if i != o_report.len() {
                report.remove(i);
            }
            let first = report.first().unwrap();
            let second = report.get(1).unwrap();
            let allowed = if second > first {
                [1, 2, 3]
            } else if second < first {
                [-1, -2, -3]
            } else {
                continue;
            };
            for (l1, l2) in report.windows(2).map(|w| (w[0], w[1])) {
                if !allowed.contains(&(i32::try_from(l2).unwrap() - i32::try_from(l1).unwrap())) {
                    continue 'outer;
                }
            }
            safe += 1;
            break;
        }
    }
    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
